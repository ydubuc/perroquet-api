use axum::{extract::State, Json};
use validator::Validate;

use crate::{app::models::api_error::ApiError, AppState};

use super::{
    dtos::{
        edit_password_dto::EditPasswordDto, refresh_access_info_dto::RefreshAccessInfoDto,
        request_email_update_dto::RequestEmailUpdateDto,
        request_password_update_dto::RequestPasswordUpdateDto, signin_apple_dto::SigninAppleDto,
        signin_dto::SigninDto, signout_dto::SignoutDto, signup_dto::SignupDto,
    },
    models::{
        access_info::AccessInfo,
        claims::{ExtractClaims, ExtractClaimsPepperEditEmail, ExtractClaimsPepperEditPassword},
    },
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

pub async fn signout(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<SignoutDto>,
) -> Result<(), ApiError> {
    dto.validate()?;
    service::signout(&dto, &claims, &state).await
}

pub async fn request_email_update(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<RequestEmailUpdateDto>,
) -> Result<(), ApiError> {
    dto.validate()?;
    service::request_email_update(&dto, &claims, &state).await
}

pub async fn process_email_update(
    State(state): State<AppState>,
    ExtractClaimsPepperEditEmail(claims): ExtractClaimsPepperEditEmail,
) -> Result<(), ApiError> {
    service::process_email_update(&claims, &state).await
}

pub async fn request_password_update(
    State(state): State<AppState>,
    Json(dto): Json<RequestPasswordUpdateDto>,
) -> Result<(), ApiError> {
    dto.validate()?;
    service::request_password_update(&dto, &state).await
}

pub async fn edit_password(
    State(state): State<AppState>,
    ExtractClaimsPepperEditPassword(claims): ExtractClaimsPepperEditPassword,
    Json(dto): Json<EditPasswordDto>,
) -> Result<(), ApiError> {
    dto.validate()?;
    service::edit_password(&dto, &claims, &state).await
}
