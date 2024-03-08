use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use cookie::{
    time::{Duration, OffsetDateTime},
    Cookie, SameSite,
};
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
        access_token_claims::{
            ExtractClaims, ExtractClaimsPepperEditEmail, ExtractClaimsPepperEditPassword,
        },
    },
    service,
};

fn cookified_access_info_response(access_info: AccessInfo) -> Response {
    let mut now = OffsetDateTime::now_utc();
    now += Duration::weeks(52);

    let mut refresh_token_cookie = Cookie::new("refresh_token", access_info.refresh_token.clone());
    refresh_token_cookie.set_same_site(SameSite::Lax);
    refresh_token_cookie.set_http_only(true);
    refresh_token_cookie.set_secure(true);
    refresh_token_cookie.set_path("/v1/auth/refresh");
    refresh_token_cookie.set_expires(now);

    let mut response = Response::builder();
    let headers = response.headers_mut().unwrap();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        "Set-Cookie",
        HeaderValue::from_str(&refresh_token_cookie.to_string()).unwrap(),
    );

    response
        .body(Body::from(serde_json::to_vec(&access_info).unwrap()))
        .unwrap()
}

pub async fn signup(
    State(state): State<AppState>,
    Json(dto): Json<SignupDto>,
) -> Result<Response, ApiError> {
    dto.validate()?;
    match service::signup(&dto, &state).await {
        Ok(data) => Ok(cookified_access_info_response(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin(
    State(state): State<AppState>,
    Json(dto): Json<SigninDto>,
) -> Result<Response, ApiError> {
    dto.validate()?;
    match service::signin(&dto, &state).await {
        Ok(data) => Ok(cookified_access_info_response(data)),
        Err(e) => Err(e),
    }
}

pub async fn signin_apple(
    State(state): State<AppState>,
    Json(dto): Json<SigninAppleDto>,
) -> Result<Response, ApiError> {
    dto.validate()?;
    match service::signin_apple(&dto, &state).await {
        Ok(data) => Ok(cookified_access_info_response(data)),
        Err(e) => Err(e),
    }
}

pub async fn refresh(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(dto): Json<Option<RefreshAccessInfoDto>>,
) -> Result<Response, ApiError> {
    if let Some(dto) = dto {
        dto.validate()?;
        tracing::debug!("received dto from body");
        match service::refresh(&dto, &state).await {
            Ok(data) => Ok(cookified_access_info_response(data)),
            Err(e) => Err(e),
        }
    } else if let Some(refresh_token) = jar.get("refresh_token") {
        tracing::debug!("received dto from cookie");
        let dto = RefreshAccessInfoDto {
            refresh_token: refresh_token.value().to_string(),
        };
        match service::refresh(&dto, &state).await {
            Ok(data) => Ok(cookified_access_info_response(data)),
            Err(e) => Err(e),
        }
    } else {
        tracing::debug!("received no dto at all");
        Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Missing refresh token.",
        ))
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
