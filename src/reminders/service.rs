use axum::http::StatusCode;
use sqlx::Postgres;

use crate::{
    app::{self, models::api_error::ApiError},
    auth::models::claims::Claims,
    AppState,
};

use super::{
    dtos::{
        create_reminder_dto::CreateReminderDto, edit_reminder_dto::EditReminderDto,
        get_reminders_filter_dto::GetRemindersFilterDto,
    },
    models::reminder::Reminder,
};

pub async fn create_reminder(
    dto: &CreateReminderDto,
    claims: &Claims,
    state: &AppState,
) -> Result<Reminder, ApiError> {
    let reminder = Reminder::new(dto, claims);

    let sqlx_result = sqlx::query(
        "
        INSERT INTO reminders
        (id, user_id, title, content, frequency, trigger_at, updated_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ",
    )
    .bind(&reminder.id)
    .bind(&claims.id)
    .bind(&reminder.title)
    .bind(&reminder.content)
    .bind(&reminder.frequency)
    .bind(&reminder.trigger_at)
    .bind(&reminder.updated_at)
    .bind(&reminder.created_at)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(_) => Ok(reminder),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create reminder.",
            ))
        }
    }
}

pub async fn get_reminders(
    dto: &GetRemindersFilterDto,
    state: &AppState,
) -> Result<Vec<Reminder>, ApiError> {
    // SQL
    let mut query = "SELECT * FROM reminders WHERE true".to_string();

    let mut sort_field = "created_at".to_string();
    let mut sort_order = "<".to_string();
    let mut page_limit: u8 = 80;

    let mut index: u8 = 0;

    if dto.id.is_some() {
        index += 1;
        query.push_str(&format!(" AND id = ${}", index));
    }

    if dto.search.is_some() {
        index += 1;
        query.push_str(&format!(" AND content LIKE ${}", index));
    }

    // SQL SORT
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
                    // add SQL clauses for pagination
                }
                Err(e) => return Err(e),
            }
        }
    }

    query.push_str(&format!(
        " ORDER BY reminders.{} {}",
        sort_field, sort_order
    ));

    if let Some(limit) = dto.limit {
        page_limit = limit;
    }
    query.push_str(&format!(" LIMIT {}", page_limit));

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Reminder>(&query);

    if let Some(id) = &dto.id {
        sqlx = sqlx.bind(id);
    }
    if let Some(search) = &dto.search {
        sqlx = sqlx.bind(search);
    }

    let sqlx_result = sqlx.fetch_all(&state.pool).await;

    match sqlx_result {
        Ok(reminders) => Ok(reminders),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get reminders.",
            ))
        }
    }
}

pub async fn get_reminder(id: &str, state: &AppState) -> Result<Reminder, ApiError> {
    let sqlx_result = sqlx::query_as::<Postgres, Reminder>(
        "
        SELECT * FROM reminders
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await;

    match sqlx_result {
        Ok(reminder) => match reminder {
            Some(reminder) => Ok(reminder),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "Reminder not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get reminder.",
            ))
        }
    }
}

pub async fn edit_reminder(
    id: &str,
    dto: &EditReminderDto,
    state: &AppState,
) -> Result<Reminder, ApiError> {
    // SQL
    let mut query = "UPDATE reminders SET ".to_string();

    let mut index: u8 = 0;

    if dto.content.is_some() {
        index += 1;
        query.push_str(&format!("content = ${}", index));
    }

    if index == 0 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Received nothing to edit.",
        ));
    }

    index += 1;
    query.push_str(&format!(" WHERE id = ${}", index));
    query.push_str(" RETURNING *");

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Reminder>(&query);

    if let Some(content) = &dto.content {
        sqlx = sqlx.bind(content);
    }
    sqlx = sqlx.bind(id);

    let sqlx_result = sqlx.fetch_optional(&state.pool).await;

    match sqlx_result {
        Ok(reminder) => match reminder {
            Some(reminder) => Ok(reminder),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "Reminder not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to edit reminder",
            ))
        }
    }
}

pub async fn delete_reminder(id: &str, state: &AppState) -> Result<(), ApiError> {
    let sqlx_result = sqlx::query(
        "
        DELETE FROM reminders
        WHERE id = $1
        ",
    )
    .bind(id)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(result) => match result.rows_affected() > 0 {
            true => Ok(()),
            false => Err(ApiError::new(StatusCode::NOT_FOUND, "Reminder not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete reminder.",
            ))
        }
    }
}
