use axum::{extract::FromRequest, Json};

use crate::app::models::api_error::ApiError;

#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiError))]
pub struct JsonFromRequest<T>(pub T);
