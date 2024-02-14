use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use sqlx::PgPool;

use crate::{app::envy::Envy, auth::authman::AuthMan};

use super::api_error::ApiError;

#[derive(Clone)]
pub struct AppState {
    pub envy: Envy,
    pub http_client: reqwest::Client,
    pub authman: AuthMan,
    pub pool: PgPool,
}

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
