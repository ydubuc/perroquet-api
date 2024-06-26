use axum::http::StatusCode;
use sqlx::Postgres;

use crate::{
    app::{self, models::api_error::ApiError},
    auth::{
        dtos::signin_dto::SigninDto, models::access_token_claims::AccessTokenClaims, util::password,
    },
    AppState,
};

use super::{dtos::get_users_filter_dto::GetUsersFilterDto, models::user::User};

pub async fn create_user(user: User, state: &AppState) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query(
        "
        INSERT INTO users (
            id, id_apple, username, username_key, email, email_key,
            password, displayname, avatar_url, updated_at, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ",
    )
    .bind(&user.id)
    .bind(&user.id_apple)
    .bind(&user.username)
    .bind(&user.username_key)
    .bind(&user.email)
    .bind(&user.email_key)
    .bind(&user.password)
    .bind(&user.displayname)
    .bind(&user.avatar_url)
    .bind(&user.updated_at)
    .bind(&user.created_at)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(_) => Ok(user),
        Err(e) => {
            let Some(db_err) = e.as_database_error() else {
                tracing::error!(%e);
                return Err(ApiError::internal_server_error());
            };
            let Some(code) = app::util::sqlx::extract_db_err_code(db_err) else {
                tracing::error!(%e);
                return Err(ApiError::internal_server_error());
            };

            match code.as_str() {
                app::util::sqlx::SqlStateCodes::UNIQUE_VIOLATION => Err(ApiError {
                    code: StatusCode::CONFLICT,
                    message: "User already exists.".to_string(),
                }),
                _ => {
                    tracing::error!(%e);
                    Err(ApiError::internal_server_error())
                }
            }
        }
    }
}

pub async fn get_users(
    dto: &GetUsersFilterDto,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Vec<User>, ApiError> {
    // SQL
    let mut query = "SELECT * FROM users WHERE true".to_string();
    let mut index: u8 = 0;

    if dto.id.is_some() {
        index += 1;
        query.push_str(&format!(" AND id = ${}", index))
    }
    if dto.search.is_some() {
        index += 1;
        query.push_str(&format!(
            " AND title LIKE ${} OR content LIKE ${}",
            index, index
        ))
    }

    // SQL SORT
    let mut sort_field = "created_at".to_string();
    let mut sort_order = "DESC".to_string();
    let limit = dto.limit.unwrap_or(100);

    if let Some(sort) = &dto.sort {
        match app::util::dto::get_sort_params(sort, None) {
            Ok(sort_params) => {
                sort_field = sort_params.field;
                sort_order = sort_params.order;
            }
            Err(e) => return Err(e),
        }

        if let Some(cursor) = &dto.cursor {
            match app::util::dto::get_cursor(&cursor) {
                Ok(cursor) => {
                    // TODO: add SQL clauses for pagination
                }
                Err(e) => return Err(e),
            }
        }
    }
    query.push_str(&format!(" ORDER BY users.{} {}", sort_field, sort_order));
    query.push_str(&format!(" LIMIT {}", limit));

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, User>(&query);

    if let Some(id) = &dto.id {
        sqlx = sqlx.bind(id);
    }
    if let Some(search) = &dto.search {
        sqlx = sqlx.bind(search);
    }

    let sqlx_result = sqlx.fetch_all(&state.pool).await;

    match sqlx_result {
        Ok(users) => Ok(users),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get users.",
            ))
        }
    }
}

pub async fn get_user_by_id(id: &str, state: &AppState) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await;

    match sqlx_result {
        Ok(data) => match data {
            Some(user) => Ok(user),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "User not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get user.",
            ))
        }
    }
}

pub async fn get_user_by_id_apple(id_apple: &str, state: &AppState) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id_apple = $1")
        .bind(id_apple)
        .fetch_optional(&state.pool)
        .await;

    match sqlx_result {
        Ok(data) => match data {
            Some(user) => Ok(user),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "User not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get user.",
            ))
        }
    }
}

pub async fn get_user_by_signin_dto(dto: &SigninDto, state: &AppState) -> Result<User, ApiError> {
    if let Some(username) = &dto.username {
        return get_user_by_username(username, state).await;
    }
    if let Some(email) = &dto.email {
        return get_user_by_email(email, state).await;
    }

    Err(ApiError::new(
        StatusCode::BAD_REQUEST,
        "Missing credentials.",
    ))
}

pub async fn get_user_by_username(username: &str, state: &AppState) -> Result<User, ApiError> {
    let sqlx_result =
        sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE username_key = $1")
            .bind(username.to_lowercase())
            .fetch_optional(&state.pool)
            .await;

    match sqlx_result {
        Ok(data) => match data {
            Some(user) => Ok(user),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "User not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get user.",
            ))
        }
    }
}

pub async fn get_user_by_email(email: &str, state: &AppState) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE email_key = $1")
        .bind(email.to_lowercase())
        .fetch_optional(&state.pool)
        .await;

    match sqlx_result {
        Ok(data) => match data {
            Some(user) => Ok(user),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "User not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get user.",
            ))
        }
    }
}

pub async fn edit_user_email_pending(
    id: &str,
    email_pending: &str,
    state: &AppState,
) -> Result<(), ApiError> {
    let sqlx_result = sqlx::query(
        "
        UPDATE users SET email_pending = $1
        WHERE id = $2
        ",
    )
    .bind(email_pending)
    .bind(id)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(result) => match result.rows_affected() > 0 {
            true => Ok(()),
            false => Err(ApiError::new(StatusCode::NOT_FOUND, "User not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::internal_server_error())
        }
    }
}

pub async fn approve_user_email_pending(id: &str, state: &AppState) -> Result<(), ApiError> {
    let sqlx_result = sqlx::query(
        "
        UPDATE users
        SET email = email_pending, email_key = LOWER(email_pending), email_pending = NULL
        WHERE id = $1 AND email_pending IS NOT NULL
        ",
    )
    .bind(id)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(result) => match result.rows_affected() > 0 {
            true => Ok(()),
            false => Err(ApiError::new(
                StatusCode::NOT_FOUND,
                "Failed to approve user email.",
            )),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::internal_server_error())
        }
    }
}

pub async fn edit_user_password(
    id: &str,
    new_password: &str,
    state: &AppState,
) -> Result<(), ApiError> {
    let Ok(password_hash) = password::hash(new_password.to_string()).await else {
        return Err(ApiError::internal_server_error());
    };

    let sqlx_result = sqlx::query(
        "
        UPDATE users SET password = $1
        WHERE id = $2
        ",
    )
    .bind(&password_hash)
    .bind(id)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(result) => match result.rows_affected() > 0 {
            true => Ok(()),
            false => Err(ApiError::new(StatusCode::NOT_FOUND, "User not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::internal_server_error())
        }
    }
}
