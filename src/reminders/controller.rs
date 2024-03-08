use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

use crate::{
    app::models::api_error::ApiError, auth::models::access_token_claims::ExtractClaims, AppState,
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
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<CreateReminderDto>,
) -> Result<Json<Reminder>, ApiError> {
    dto.validate()?;
    match service::create_reminder(&dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_reminders(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Query(dto): Query<GetRemindersFilterDto>,
) -> Result<Json<Vec<Reminder>>, ApiError> {
    dto.validate()?;
    match service::get_reminders(&dto, Some(&claims), &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_reminder(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Path(id): Path<String>,
) -> Result<Json<Reminder>, ApiError> {
    match service::get_reminder(&id, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn edit_reminder(
    State(state): State<AppState>,
    Path(id): Path<String>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<EditReminderDto>,
) -> Result<Json<Reminder>, ApiError> {
    dto.validate()?;
    match service::edit_reminder(&id, &dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn delete_reminder(
    State(state): State<AppState>,
    Path(id): Path<String>,
    ExtractClaims(claims): ExtractClaims,
) -> Result<(), ApiError> {
    return service::delete_reminder(&id, &claims, &state).await;
}
