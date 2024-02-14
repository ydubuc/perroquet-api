use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, HeaderMap, StatusCode},
    RequestPartsExt,
};
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{app::models::api_error::ApiError, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub iat: u64,
    pub exp: u64,
}

impl Claims {
    fn from_headers(headers: &HeaderMap, secret: &str) -> Result<Self, ApiError> {
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

        let access_token = split[1];
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        match jsonwebtoken::decode::<Claims>(&access_token, &decoding_key, &validation) {
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

pub struct ExtractClaims(pub Claims);

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

        match Claims::from_headers(headers, &state.envy.jwt_secret) {
            Ok(claims) => Ok(ExtractClaims(claims)),
            Err(e) => Err(e),
        }
    }
}
