use crate::AppState;

use super::models::api_error::ApiError;

pub async fn get_root(state: AppState) -> Result<String, ApiError> {
    let response = format!("Hello, World! -from {}", state.envy.app_env);
    Ok(response)
}
