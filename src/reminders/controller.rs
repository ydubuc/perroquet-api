use axum::{extract::State, Json};

use crate::{
    app::{models::api_error::ApiError, structs::json_from_request::JsonFromRequest},
    AppState,
};

use super::{
    dtos::{
        create_reminder_dto::CreateReminderDto, get_reminders_filter_dto::GetRemindersFilterDto,
    },
    models::reminder::Reminder,
    service,
};

pub async fn create_reminder(
    State(state): State<AppState>,
    JsonFromRequest(dto): JsonFromRequest<CreateReminderDto>,
) -> Result<Json<Reminder>, ApiError> {
    match service::create_reminder(state, dto).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_reminders(
    State(state): State<AppState>,
    JsonFromRequest(dto): JsonFromRequest<GetRemindersFilterDto>,
) -> Result<Json<Vec<Reminder>>, ApiError> {
    match service::get_reminders(state, dto).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
