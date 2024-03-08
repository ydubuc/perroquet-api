use crate::{
    auth::models::access_token_claims::AccessTokenClaims, reminders::models::reminder::Reminder,
    users::models::user::User, AppState,
};

use super::{
    dtos::sync_dto::SyncDto,
    models::{api_error::ApiError, sync_data::SyncData},
};

pub async fn get_root(state: &AppState) -> Result<String, ApiError> {
    let response = format!("Hello, World! -from {}", state.envy.app_env);
    Ok(response)
}

pub async fn sync(
    dto: &SyncDto,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<SyncData, ApiError> {
    let mut user: Option<User> = None;
    let mut reminders: Option<Vec<Reminder>> = None;

    Ok(SyncData { user, reminders })
}
