use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

use crate::{
    app::{models::api_error::ApiError, structs::json_from_request::JsonFromRequest},
    auth::models::claims::Claims,
    AppState,
};

use super::{
    dtos::{
        create_reminder_dto::CreateReminderDto, edit_reminder_dto::EditReminderDto,
        get_reminders_filter_dto::GetRemindersFilterDto,
    },
    models::reminder::Reminder,
    service,
};

pub async fn create_reminder(
    State(state): State<AppState>,
    headers: HeaderMap,
    JsonFromRequest(dto): JsonFromRequest<CreateReminderDto>,
) -> Result<Json<Reminder>, ApiError> {
    let claims = Claims::from_headers(headers, &state.envy.jwt_secret)?;

    match service::create_reminder(&dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_reminders(
    State(state): State<AppState>,
    JsonFromRequest(dto): JsonFromRequest<GetRemindersFilterDto>,
) -> Result<Json<Vec<Reminder>>, ApiError> {
    match service::get_reminders(&dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_reminder(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Reminder>, ApiError> {
    match service::get_reminder(&id, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn edit_reminder(
    State(state): State<AppState>,
    Path(id): Path<String>,
    JsonFromRequest(dto): JsonFromRequest<EditReminderDto>,
) -> Result<Json<Reminder>, ApiError> {
    match service::edit_reminder(&id, &dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn delete_reminder(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(), ApiError> {
    return service::delete_reminder(&id, &state).await;
}
