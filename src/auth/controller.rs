use axum::{extract::State, Json};
use validator::Validate;

use crate::{app::models::api_error::ApiError, AppState};

use super::{
    dtos::{
        refresh_access_info_dto::RefreshAccessInfoDto, signin_apple_dto::SigninAppleDto,
        signin_dto::SigninDto, signup_dto::SignupDto,
    },
    models::access_info::AccessInfo,
    service,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(dto): Json<SignupDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    dto.validate()?;
    match service::signup(&dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin(
    State(state): State<AppState>,
    Json(dto): Json<SigninDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    dto.validate()?;
    match service::signin(&dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin_apple(
    State(state): State<AppState>,
    Json(dto): Json<SigninAppleDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    dto.validate()?;
    match service::signin_apple(&dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(dto): Json<RefreshAccessInfoDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    dto.validate()?;
    match service::refresh(&dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
