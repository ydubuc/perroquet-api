use axum::{extract::State, Json};

use crate::{
    app::{models::api_error::ApiError, structs::json_from_request::JsonFromRequest},
    AppState,
};

use super::{
    dtos::{signin_apple_dto::SigninAppleDto, signin_dto::SigninDto, signup_dto::SignupDto},
    models::access_info::AccessInfo,
    service,
};

pub async fn signup(
    State(state): State<AppState>,
    JsonFromRequest(dto): JsonFromRequest<SignupDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::signup(&dto, &None, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin(
    State(state): State<AppState>,
    JsonFromRequest(dto): JsonFromRequest<SigninDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::signin(&dto, true, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin_apple(
    State(state): State<AppState>,
    JsonFromRequest(dto): JsonFromRequest<SigninAppleDto>,
) -> Result<Json<AccessInfo>, ApiError> {
    match service::signin_apple(&dto, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
