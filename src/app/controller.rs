use axum::extract::State;

use crate::{auth::models::claims::ExtractClaims, AppState};

use super::{models::api_error::ApiError, service};

pub async fn get_root(State(state): State<AppState>) -> Result<String, ApiError> {
    return service::get_root(&state).await;
}

pub async fn sync(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
) -> Result<(), ApiError> {
    return service::sync(&claims, &state).await;
}
