use axum::http::StatusCode;
use sqlx::Postgres;

use crate::{
    app::{self, models::api_error::ApiError},
    auth::dtos::signin_dto::SigninDto,
    AppState,
};

use super::models::user::User;

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

pub async fn get_user_by_id(id: &str, state: &AppState) -> Result<User, ApiError> {
    let sqlx_result = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await;

    match sqlx_result {
        Ok(user) => match user {
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
        Ok(user) => match user {
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
        Ok(user) => match user {
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
