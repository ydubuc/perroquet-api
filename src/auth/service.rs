use axum::http::StatusCode;

use crate::{
    app::models::api_error::ApiError,
    users::{self, models::user::User},
    AppState,
};

use super::{
    dtos::{signin_apple_dto::SigninAppleDto, signin_dto::SigninDto, signup_dto::SignupDto},
    jwt,
    models::access_info::AccessInfo,
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
    // TODO: create device
    let access_info = AccessInfo {
        access_token: jwt::service::sign_jwt(&user, &state.envy.jwt_secret, None),
        refresh_token: None,
        device_id: None,
    };

    Ok(access_info)
}
