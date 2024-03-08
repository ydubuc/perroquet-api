use axum::http::StatusCode;
use sqlx::Postgres;

use crate::{
    app::{self, models::api_error::ApiError, util::time},
    auth::models::access_token_claims::AccessTokenClaims,
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
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Reminder, ApiError> {
    let reminder = Reminder::new(dto, claims);

    let sqlx_result = sqlx::query(
        "
        INSERT INTO reminders
        (id, user_id, title, description, tags, frequency, visibility, trigger_at, updated_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ",
    )
    .bind(&reminder.id)
    .bind(&claims.id)
    .bind(&reminder.title)
    .bind(&reminder.description)
    .bind(&reminder.tags)
    .bind(&reminder.frequency)
    .bind(&reminder.visibility)
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
    claims: Option<&AccessTokenClaims>,
    state: &AppState,
) -> Result<Vec<Reminder>, ApiError> {
    // SQL
    let mut query = "SELECT * FROM reminders WHERE true".to_string();

    let mut sort_field = "trigger_at".to_string();
    let mut sort_order = "DESC".to_string();
    let mut page_limit: u8 = 80;

    let mut index: u8 = 0;

    if dto.id.is_some() {
        index += 1;
        query.push_str(&format!(" AND id = ${}", index));
    }
    if dto.search.is_some() {
        index += 1;
        query.push_str(&format!(" AND title LIKE ${}", index));
    }
    if dto.tags.is_some() {
        let tags = dto.tags.clone().unwrap();
        let mut values = String::new();

        for (i, value) in tags.split(',').enumerate() {
            index += 1;
            if i != 0 {
                values.push_str(", ");
            }
            values.push_str(&format!("${}", index));
        }

        query.push_str(&format!(" AND tags @> ARRAY[{}]", values));
    }
    if dto.visibility.is_some() {
        index += 1;
        query.push_str(&format!(" AND visibility = ${}", index));
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
    if let Some(limit) = dto.limit {
        page_limit = limit;
    }
    query.push_str(&format!(" LIMIT {}", page_limit));

    tracing::debug!("{:?}", query);

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Reminder>(&query);

    if let Some(id) = &dto.id {
        sqlx = sqlx.bind(id);
    }
    if let Some(search) = &dto.search {
        sqlx = sqlx.bind(search);
    }
    if let Some(tags) = &dto.tags {
        for tag in tags.split(",") {
            sqlx = sqlx.bind(tag);
        }
    }
    if let Some(visibility) = &dto.visibility {
        sqlx = sqlx.bind(visibility);
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

pub async fn get_reminder(
    id: &str,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Reminder, ApiError> {
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
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Reminder, ApiError> {
    // SQL
    let mut query = "UPDATE reminders SET ".to_string();

    let mut index: u8 = 0;

    if dto.title.is_some() {
        index += 1;
        query.push_str(&format!("title = ${}, ", index));
    }
    if dto.description.is_some() {
        index += 1;
        query.push_str(&format!("description = ${}, ", index));
    }
    if dto.tags.is_some() {
        index += 1;
        query.push_str(&format!("tags = ${}, ", index));
    }
    if dto.frequency.is_some() {
        index += 1;
        query.push_str(&format!("frequency = ${}, ", index));
    }
    if dto.visibility.is_some() {
        index += 1;
        query.push_str(&format!("visibility = ${}, ", index));
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
    let mut sqlx = sqlx::query_as::<Postgres, Reminder>(&query);

    if let Some(title) = &dto.title {
        sqlx = sqlx.bind(title)
    }
    if let Some(description) = &dto.description {
        sqlx = sqlx.bind(description);
    }
    if let Some(tags) = &dto.tags {
        sqlx = sqlx.bind(tags);
    }
    if let Some(frequency) = &dto.frequency {
        sqlx = sqlx.bind(frequency);
    }
    if let Some(visibility) = &dto.visibility {
        sqlx = sqlx.bind(visibility);
    }
    if let Some(trigger_at) = &dto.trigger_at {
        sqlx = sqlx.bind(trigger_at);
    }
    sqlx = sqlx.bind(time::current_time_in_millis());
    sqlx = sqlx.bind(id);
    sqlx = sqlx.bind(&claims.id);

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

pub async fn delete_reminder(
    id: &str,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<(), ApiError> {
    let sqlx_result = sqlx::query(
        "
        DELETE FROM reminders
        WHERE id = $1 AND user_id = $2
        ",
    )
    .bind(id)
    .bind(&claims.id)
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
