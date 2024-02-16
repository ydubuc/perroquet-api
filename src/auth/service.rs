use axum::http::StatusCode;

use crate::{
    app::models::api_error::ApiError,
    devices,
    users::{self, models::user::User},
    AppState,
};

use super::{
    dtos::{
        refresh_access_info_dto::RefreshAccessInfoDto, signin_apple_dto::SigninAppleDto,
        signin_dto::SigninDto, signup_dto::SignupDto,
    },
    models::{access_info::AccessInfo, claims::AccessTokenClaims},
    util::password,
};

pub async fn signup(dto: &SignupDto, state: &AppState) -> Result<AccessInfo, ApiError> {
    let Ok(password_hash) = password::hash(dto.password.to_string()).await else {
        return Err(ApiError::internal_server_error());
    };

    let user = User::new(&dto.username, &dto.email, &Some(password_hash), &None);

    match users::service::create_user(user, state).await {
        Ok(user) => signin_user(&user, state).await,
        Err(e) => Err(e),
    }
}

async fn signup_apple(
    email: &str,
    id_apple: &str,
    state: &AppState,
) -> Result<AccessInfo, ApiError> {
    let user = User::new(&None, email, &None, &Some(id_apple.to_string()));

    match users::service::create_user(user, state).await {
        Ok(user) => signin_user(&user, state).await,
        Err(e) => Err(e),
    }
}

pub async fn signin(dto: &SigninDto, state: &AppState) -> Result<AccessInfo, ApiError> {
    let user_result = users::service::get_user_by_signin_dto(dto, state).await;
    let Ok(user) = user_result else {
        // TODO: return sleep time error
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid credentials.",
        ));
    };
    let Some(user_password) = &user.password else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid credentials.",
        ));
    };
    let Ok(matches) = password::verify(dto.password.to_string(), user_password.to_string()).await
    else {
        // TODO: return sleep time error
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid credentials.",
        ));
    };
    if !matches {
        // TODO: return sleep time error
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid credentials.",
        ));
    }

    signin_user(&user, state).await
}

pub async fn signin_apple(dto: &SigninAppleDto, state: &AppState) -> Result<AccessInfo, ApiError> {
    let _apple_client = state.authman.apple_client(&state.http_client).await;
    let apple_client = _apple_client.read().await;
    let auth_code_res = apple_client
        .validate_auth_code(&dto.auth_code, &state.http_client)
        .await?;

    let Ok(claims) =
        auth_code_res.decode_id_token(&apple_client.public_keys, &apple_client.config.client_id)
    else {
        return Err(ApiError::internal_server_error());
    };

    match users::service::get_user_by_email(&claims.email, state).await {
        Ok(user) => signin_user(&user, state).await,
        Err(e) => match e.code {
            StatusCode::NOT_FOUND => signup_apple(&claims.email, &claims.sub, state).await,
            _ => Err(ApiError::internal_server_error()),
        },
    }
}

async fn signin_user(user: &User, state: &AppState) -> Result<AccessInfo, ApiError> {
    let claims = AccessTokenClaims::new(&user.id.to_string());
    let Ok(access_token) = claims.to_jwt(&state.envy.jwt_secret, None) else {
        return Err(ApiError::internal_server_error());
    };

    let Ok(device) = devices::service::create_device(user, state).await else {
        return Err(ApiError::internal_server_error());
    };

    let access_info = AccessInfo {
        access_token,
        refresh_token: Some(device.refresh_token),
        device_id: Some(device.id.to_string()),
    };

    Ok(access_info)
}

pub async fn refresh(dto: &RefreshAccessInfoDto, state: &AppState) -> Result<AccessInfo, ApiError> {
    let refresh_device_result = devices::service::refresh_device(&dto.refresh_token, state).await;

    match refresh_device_result {
        Ok(device) => {
            let claims = AccessTokenClaims::new(&device.user_id.to_string());
            let Ok(access_token) = claims.to_jwt(&state.envy.jwt_secret, None) else {
                return Err(ApiError::internal_server_error());
            };

            Ok(AccessInfo {
                access_token,
                refresh_token: Some(device.refresh_token),
                device_id: Some(device.id.to_string()),
            })
        }
        Err(e) => match e.code {
            StatusCode::NOT_FOUND => Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "Device not found or expired.",
            )),
            _ => Err(e),
        },
    }
}
