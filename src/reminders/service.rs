use axum::http::StatusCode;
use sqlx::Postgres;

use crate::{app::models::api_error::ApiError, AppState};

use super::{
    dtos::{
        create_reminder_dto::CreateReminderDto, get_reminders_filter_dto::GetRemindersFilterDto,
    },
    models::reminder::Reminder,
};

pub async fn create_reminder(
    state: AppState,
    dto: CreateReminderDto,
) -> Result<Reminder, ApiError> {
    let reminder = Reminder::new(dto.body);

    tracing::debug!("{:?}", reminder);

    let sqlx_result = sqlx::query(
        "
        INSERT INTO reminders
        (id, body, created_at)
        VALUES ($1, $2, $3)
        ",
    )
    .bind(&reminder.id)
    .bind(&reminder.body)
    .bind(&reminder.created_at)
    .execute(&state.db_pool)
    .await;

    match sqlx_result {
        Ok(_) => Ok(reminder),
        Err(e) => {
            tracing::error!(%e);
            return Err(ApiError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to create reminder.".to_owned(),
            });
        }
    }
}

pub async fn get_reminders(
    state: AppState,
    dto: GetRemindersFilterDto,
) -> Result<Vec<Reminder>, ApiError> {
    // SQL
    let mut query = "SELECT * FROM reminders WHERE true".to_string();

    let mut index: u8 = 0;

    if dto.id.is_some() {
        index += 1;
        query.push_str(&format!(" AND id = ${}", index));
    }

    if dto.search.is_some() {
        index += 1;
        query.push_str(&format!(" AND body LIKE ${} ", index));
    }

    if let Some(limit) = dto.limit {
        query.push_str(&format!("LIMIT {}", limit));
    }

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Reminder>(&query);

    if let Some(id) = dto.id {
        sqlx = sqlx.bind(id);
    }
    if let Some(search) = dto.search {
        sqlx = sqlx.bind(search);
    }

    match sqlx.fetch_all(&state.db_pool).await {
        Ok(reminders) => Ok(reminders),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to get reminders.".to_string(),
            })
        }
    }
}
