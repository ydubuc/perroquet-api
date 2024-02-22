use axum::{extract::State, Json};
use validator::Validate;

use crate::{auth::models::claims::ExtractClaims, AppState};

use super::{
    dtos::sync_dto::SyncDto,
    models::{api_error::ApiError, sync_data::SyncData},
    service,
};

pub async fn get_root(State(state): State<AppState>) -> Result<String, ApiError> {
    return service::get_root(&state).await;
}

pub async fn sync(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<SyncDto>,
) -> Result<Json<SyncData>, ApiError> {
    dto.validate()?;
    match service::sync(&dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
