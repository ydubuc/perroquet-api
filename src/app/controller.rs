use axum::extract::State;

use crate::AppState;

use super::{models::api_error::ApiError, service};

pub async fn get_root(State(state): State<AppState>) -> Result<String, ApiError> {
    return service::get_root(state).await;
}
