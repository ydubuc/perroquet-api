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
    let Some(password) = &dto.password else {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Missing password."));
    };
    let Ok(password_hash) = password::hash(password.to_string()).await else {
        return Err(ApiError::internal_server_error());
    };

    let user = User::new(&dto.username, &dto.email, &Some(password_hash), &None);

    tracing::info!("{:?}", user);

    match users::service::create_user(user, state).await {
        Ok(_) => {
            let signin_dto = SigninDto {
                username: None,
                email: Some(dto.email.to_owned()),
                password: dto.password.to_owned(),
            };

            signin(&signin_dto, true, state).await
        }
        Err(e) => Err(e),
    }
}

async fn signup_apple(
    dto: &SignupDto,
    id_apple: &str,
    state: &AppState,
) -> Result<AccessInfo, ApiError> {
    let user = User::new(
        &dto.username,
        &dto.email,
        &None,
        &Some(id_apple.to_string()),
    );

    tracing::info!("{:?}", user);

    match users::service::create_user(user, state).await {
        Ok(_) => {
            let signin_dto = SigninDto {
                username: None,
                email: Some(dto.email.to_owned()),
                password: None,
            };

            signin(&signin_dto, false, state).await
        }
        Err(e) => Err(e),
    }
}

pub async fn signin(
    dto: &SigninDto,
    verify_password: bool,
    state: &AppState,
) -> Result<AccessInfo, ApiError> {
    let user_result = users::service::get_user_by_signin_dto(dto, state).await;
    let Ok(user) = user_result else {
        // TODO: return sleep time error
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid credentials.",
        ));
    };

    if verify_password {
        let Some(dto_password) = &dto.password else {
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
        let Ok(matches) =
            password::verify(dto_password.to_string(), user_password.to_string()).await
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
    }

    // TODO: create device
    let access_info = AccessInfo {
        access_token: jwt::service::sign_jwt(&user, &state.envy.jwt_secret, None),
        refresh_token: None,
        device_id: None,
    };

    Ok(access_info)
}

pub async fn signin_apple(dto: &SigninAppleDto, state: &AppState) -> Result<AccessInfo, ApiError> {
    let _apple_client = state.authman.apple_client(&state.http_client).await;
    let apple_client = _apple_client.read().await;
    let auth_code_res = apple_client
        .validate_auth_code(&dto.auth_code, &state.http_client)
        .await?;

    let Ok(id_token_claims) =
        auth_code_res.decode_id_token(&apple_client.public_keys, &apple_client.config.client_id)
    else {
        return Err(ApiError::internal_server_error());
    };

    match users::service::get_user_by_email(&id_token_claims.email, &state).await {
        Ok(user) => {
            let dto = SigninDto {
                username: None,
                email: Some(user.email),
                password: None,
            };

            return signin(&dto, false, &state).await;
        }
        Err(e) => {
            if e.code != StatusCode::NOT_FOUND {
                return Err(ApiError::internal_server_error());
            }

            let dto = SignupDto {
                username: None,
                email: id_token_claims.email,
                password: None,
            };

            return signup_apple(&dto, &id_token_claims.sub, &state).await;
        }
    }
}
