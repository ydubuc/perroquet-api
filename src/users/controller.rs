use axum::{
    extract::{Query, State},
    Json,
};
use validator::Validate;

use crate::{
    app::models::{api_error::ApiError, app_state::AppState},
    auth::models::claims::ExtractClaims,
};

use super::{dtos::get_users_filter_dto::GetUsersFilterDto, models::user::User, service};

pub async fn get_users(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Query(dto): Query<GetUsersFilterDto>,
) -> Result<Json<Vec<User>>, ApiError> {
    dto.validate()?;
    match service::get_users(&dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_me(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
) -> Result<Json<User>, ApiError> {
    match service::get_user_by_id(&claims.id, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
