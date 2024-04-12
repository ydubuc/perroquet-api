use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, HeaderMap, StatusCode},
    RequestPartsExt,
};
use jsonwebtoken::{
    encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{
        models::{api_error::ApiError, app_error::AppError},
        util::time,
    },
    auth::{config::JWT_EXP, enums::pepper_type::PepperType},
    AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub id: String,
    pub iat: u64,
    pub exp: u64,
}

impl AccessTokenClaims {
    pub fn new(id: &str) -> Self {
        let iat = time::current_time_in_secs() as u64;
        let exp = iat + JWT_EXP;

        Self {
            id: id.to_string(),
            iat,
            exp,
        }
    }

    pub fn to_jwt(self, secret: &str, pepper: Option<&str>) -> Result<String, AppError> {
        let mut secret = secret.to_string();
        if let Some(pepper) = pepper {
            secret = [&secret, pepper].concat();
        }

        let encode_result = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        );

        match encode_result {
            Ok(jwt) => Ok(jwt),
            Err(e) => {
                tracing::error!(%e);
                Err(AppError::new("failed to encode claims"))
            }
        }
    }

    fn from_headers(
        headers: &HeaderMap,
        secret: &str,
        pepper: Option<&str>,
    ) -> Result<Self, ApiError> {
        let Some(header_value) = headers.get(AUTHORIZATION) else {
            return Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "Missing access token.",
            ));
        };

        let Ok(bearer) = header_value.to_str() else {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve Authorization header.",
            ));
        };

        let split: Vec<&str> = bearer.split(" ").collect();
        if split.len() != 2 || split[0] != "Bearer" {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "Authorization must be Bearer.",
            ));
        }

        AccessTokenClaims::from_jwt(split[1], secret, pepper, true)
    }

    pub fn from_jwt(
        jwt: &str,
        secret: &str,
        pepper: Option<&str>,
        validate_exp: bool,
    ) -> Result<Self, ApiError> {
        let mut secret = secret.to_string();
        if let Some(pepper) = pepper {
            secret = [&secret, pepper].concat();
        }

        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = validate_exp;

        let decode_result =
            jsonwebtoken::decode::<AccessTokenClaims>(&jwt, &decoding_key, &validation);

        match decode_result {
            Ok(data) => Ok(data.claims),
            Err(e) => match e.kind() {
                ErrorKind::ExpiredSignature => {
                    Err(ApiError::new(StatusCode::UNAUTHORIZED, "Token expired."))
                }
                _ => Err(ApiError::new(StatusCode::UNAUTHORIZED, "Invalid token.")),
            },
        }
    }
}

pub struct ExtractClaims(pub AccessTokenClaims);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = parts.extract_with_state::<AppState, _>(state).await?;
        let headers = &parts.headers;

        match AccessTokenClaims::from_headers(headers, &state.envy.jwt_secret, None) {
            Ok(claims) => Ok(ExtractClaims(claims)),
            Err(e) => Err(e),
        }
    }
}

pub struct ExtractClaimsPepperVerifyEmail(pub AccessTokenClaims);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractClaimsPepperVerifyEmail
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = parts.extract_with_state::<AppState, _>(state).await?;
        let headers = &parts.headers;

        match AccessTokenClaims::from_headers(
            headers,
            &state.envy.jwt_secret,
            Some(PepperType::VERIFY_EMAIL),
        ) {
            Ok(claims) => Ok(ExtractClaimsPepperVerifyEmail(claims)),
            Err(e) => Err(e),
        }
    }
}

pub struct ExtractClaimsPepperEditEmail(pub AccessTokenClaims);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractClaimsPepperEditEmail
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = parts.extract_with_state::<AppState, _>(state).await?;
        let headers = &parts.headers;

        match AccessTokenClaims::from_headers(
            headers,
            &state.envy.jwt_secret,
            Some(PepperType::EDIT_EMAIL),
        ) {
            Ok(claims) => Ok(ExtractClaimsPepperEditEmail(claims)),
            Err(e) => Err(e),
        }
    }
}

pub struct ExtractClaimsPepperEditPassword(pub AccessTokenClaims);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractClaimsPepperEditPassword
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = parts.extract_with_state::<AppState, _>(state).await?;
        let headers = &parts.headers;

        match AccessTokenClaims::from_headers(
            headers,
            &state.envy.jwt_secret,
            Some(PepperType::EDIT_PASSWORD),
        ) {
            Ok(claims) => Ok(ExtractClaimsPepperEditPassword(claims)),
            Err(e) => Err(e),
        }
    }
}
