use axum::{extract::State, Json};

use crate::{app::models::api_error::ApiError, AppState};

use super::{
    dtos::{signin_apple_dto::SigninAppleDto, signin_dto::SigninDto, signup_dto::SignupDto},
    models::access_info::AccessInfo,
    service,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(dto): Json<SignupDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::signup(&dto, &None, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin(
    State(state): State<AppState>,
    Json(dto): Json<SigninDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::signin(&dto, true, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin_apple(
    State(state): State<AppState>,
    Json(dto): Json<SigninAppleDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::signin_apple(&dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
