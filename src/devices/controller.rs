use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

use crate::{
    app::models::{api_error::ApiError, app_state::AppState},
    auth::models::access_token_claims::ExtractClaims,
};

use super::{
    dtos::{edit_device_dto::EditDeviceDto, get_devices_filter_dto::GetDevicesFilterDto},
    models::device::Device,
    service,
};

pub async fn get_devices(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Query(dto): Query<GetDevicesFilterDto>,
) -> Result<Json<Vec<Device>>, ApiError> {
    dto.validate()?;
    match service::get_devices(&dto, Some(&claims), &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn edit_device(
    State(state): State<AppState>,
    Path(id): Path<String>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<EditDeviceDto>,
) -> Result<Json<Device>, ApiError> {
    dto.validate()?;
    match service::edit_device(&id, &dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
