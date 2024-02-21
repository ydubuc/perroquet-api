use axum::{extract::State, Json};

use crate::{auth::models::claims::ExtractClaims, AppState};

use super::{dtos::sync_dto::SyncDto, models::api_error::ApiError, service};

pub async fn get_root(State(state): State<AppState>) -> Result<String, ApiError> {
    return service::get_root(&state).await;
}

pub async fn sync(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<SyncDto>,
) -> Result<(), ApiError> {
    return service::sync(&dto, &claims, &state).await;
}
