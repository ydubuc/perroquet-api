use axum::http::StatusCode;
use sqlx::Postgres;

use crate::{
    app::{self, models::api_error::ApiError, util::time},
    auth::models::access_token_claims::AccessTokenClaims,
    AppState,
};

use super::{
    dtos::{
        create_memo_dto::CreateMemoDto, edit_memo_dto::EditMemoDto, get_memos_dto::GetMemosDto,
    },
    models::memo::Memo,
};

pub async fn create_memo(
    dto: &CreateMemoDto,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Memo, ApiError> {
    let memo = Memo::new(dto, claims);

    let sqlx_result = sqlx::query(
        "
        INSERT INTO memos
        (id, user_id, title, description, priority, status, visibility, frequency, trigger_at, updated_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ",
    )
    .bind(&memo.id)
    .bind(&claims.id)
    .bind(&memo.title)
    .bind(&memo.description)
    .bind(&memo.priority)
    .bind(&memo.status)
    .bind(&memo.visibility)
    .bind(&memo.frequency)
    .bind(&memo.trigger_at)
    .bind(&memo.updated_at)
    .bind(&memo.created_at)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(_) => Ok(memo),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create memo.",
            ))
        }
    }
}

pub async fn get_memos(
    dto: &GetMemosDto,
    claims: Option<&AccessTokenClaims>,
    state: &AppState,
) -> Result<Vec<Memo>, ApiError> {
    // SQL
    let mut query = "SELECT * FROM memos WHERE true".to_string();
    let mut index: u8 = 0;

    if dto.id.is_some() {
        index += 1;
        query.push_str(&format!(" AND id = ${}", index));
    }
    if dto.user_id.is_some() {
        index += 1;
        query.push_str(&format!(" AND user_id = ${}", index));
    }
    if dto.search.is_some() {
        index += 1;
        query.push_str(&format!(" AND title LIKE ${}", index));
    }
    if dto.priority.is_some() {
        index += 1;
        query.push_str(&format!(" AND priority = ${}", index));
    }
    if dto.status.is_some() {
        index += 1;
        query.push_str(&format!(" AND status = ${}", index));
    }
    if dto.visibility.is_some() {
        index += 1;
        query.push_str(&format!(" AND visibility = ${}", index));
    }

    // SQL SORT
    let mut sort_field = "trigger_at".to_string();
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
                    let carrot_sign = match sort_order.as_ref() {
                        "DESC" => "<",
                        _ => ">",
                    };
                    query.push_str(&format!(
                        " AND ({}, id) {} ({}, '{}')",
                        sort_field, carrot_sign, cursor.value, cursor.id
                    ))
                }
                Err(e) => return Err(e),
            }
        }
    }
    query.push_str(&format!(
        " ORDER BY {} {}, id {}",
        sort_field, sort_order, sort_order
    ));
    query.push_str(&format!(" LIMIT {}", limit));

    tracing::debug!("{:?}", query);

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Memo>(&query);

    if let Some(id) = &dto.id {
        sqlx = sqlx.bind(id);
    }
    if let Some(user_id) = &dto.user_id {
        sqlx = sqlx.bind(user_id);
    }
    if let Some(search) = &dto.search {
        sqlx = sqlx.bind(search);
    }
    if let Some(priority) = &dto.priority {
        sqlx = sqlx.bind(priority)
    }
    if let Some(status) = &dto.status {
        sqlx = sqlx.bind(status);
    }
    if let Some(visibility) = &dto.visibility {
        sqlx = sqlx.bind(visibility);
    }

    let sqlx_result = sqlx.fetch_all(&state.pool).await;

    match sqlx_result {
        Ok(memos) => Ok(memos),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get memos.",
            ))
        }
    }
}

pub async fn get_memo(
    id: &str,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Memo, ApiError> {
    let sqlx_result = sqlx::query_as::<Postgres, Memo>(
        "
        SELECT * FROM memos
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await;

    match sqlx_result {
        Ok(data) => match data {
            Some(memo) => Ok(memo),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "Memo not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get memo.",
            ))
        }
    }
}

pub async fn edit_memo(
    id: &str,
    dto: &EditMemoDto,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Memo, ApiError> {
    // SQL
    let mut query = "UPDATE memos SET ".to_string();
    let mut index: u8 = 0;

    if dto.title.is_some() {
        index += 1;
        query.push_str(&format!("title = ${}, ", index));
    }
    if dto.description.is_some() {
        index += 1;
        query.push_str(&format!("description = ${}, ", index));
    }
    if dto.priority.is_some() {
        index += 1;
        query.push_str(&format!("priority = ${}, ", index));
    }
    if dto.status.is_some() {
        index += 1;
        query.push_str(&format!("status = ${}, ", index));
    }
    if dto.visibility.is_some() {
        index += 1;
        query.push_str(&format!("visibility = ${}, ", index));
    }
    if dto.frequency.is_some() {
        index += 1;
        query.push_str(&format!("frequency = ${}, ", index));
    }
    if dto.trigger_at.is_some() {
        index += 1;
        query.push_str(&format!("trigger_at = ${}, ", index));
    }

    index += 1;
    query.push_str(&format!("updated_at = ${} ", index));
    index += 1;
    query.push_str(&format!("WHERE id = ${} ", index));
    index += 1;
    query.push_str(&format!("AND user_id = ${} ", index));
    query.push_str("RETURNING *");

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Memo>(&query);

    if let Some(title) = &dto.title {
        sqlx = sqlx.bind(title)
    }
    if let Some(description) = &dto.description {
        sqlx = sqlx.bind(description);
    }
    if let Some(priority) = &dto.priority {
        sqlx = sqlx.bind(priority);
    }
    if let Some(status) = &dto.status {
        sqlx = sqlx.bind(status);
    }
    if let Some(visibility) = &dto.visibility {
        sqlx = sqlx.bind(visibility);
    }
    if let Some(frequency) = &dto.frequency {
        sqlx = sqlx.bind(frequency);
    }
    if let Some(trigger_at) = &dto.trigger_at {
        sqlx = sqlx.bind(trigger_at);
    }
    sqlx = sqlx.bind(time::current_time_in_millis());
    sqlx = sqlx.bind(id);
    sqlx = sqlx.bind(&claims.id);

    let sqlx_result = sqlx.fetch_optional(&state.pool).await;

    match sqlx_result {
        Ok(data) => match data {
            Some(memo) => Ok(memo),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "Memo not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to edit memo",
            ))
        }
    }
}
