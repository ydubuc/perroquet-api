use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

use crate::{
    app::models::api_error::ApiError, auth::models::access_token_claims::ExtractClaims, AppState,
};

use super::{
    dtos::{
        create_memo_dto::CreateMemoDto, edit_memo_dto::EditMemoDto, get_memos_dto::GetMemosDto,
    },
    models::memo::Memo,
    service,
};

pub async fn create_memo(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<CreateMemoDto>,
) -> Result<Json<Memo>, ApiError> {
    dto.validate()?;
    match service::create_memo(&dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_memos(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Query(dto): Query<GetMemosDto>,
) -> Result<Json<Vec<Memo>>, ApiError> {
    dto.validate()?;
    match service::get_memos(&dto, Some(&claims), &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn get_memo(
    State(state): State<AppState>,
    ExtractClaims(claims): ExtractClaims,
    Path(id): Path<String>,
) -> Result<Json<Memo>, ApiError> {
    match service::get_memo(&id, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub async fn edit_memo(
    State(state): State<AppState>,
    Path(id): Path<String>,
    ExtractClaims(claims): ExtractClaims,
    Json(dto): Json<EditMemoDto>,
) -> Result<Json<Memo>, ApiError> {
    dto.validate()?;
    match service::edit_memo(&id, &dto, &claims, &state).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}
