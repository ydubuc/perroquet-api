use std::borrow::Cow;

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use validator::{ValidationError, ValidationErrors};

#[derive(Debug)]
pub struct ApiError {
    pub code: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(code: StatusCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "An error occurred.".to_string(),
        }
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        let code = match rejection {
            JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
            JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            code,
            message: rejection.to_string(),
        }
    }
}

impl From<ValidationError> for ApiError {
    fn from(error: ValidationError) -> Self {
        let message = error
            .message
            .unwrap_or(Cow::from("Invalid input."))
            .to_string();

        Self {
            code: StatusCode::BAD_REQUEST,
            message,
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let mut message = "".to_string();
        let field_errors = errors.field_errors();

        for (_, value) in field_errors {
            for error in value {
                if let Some(error_message) = &error.message {
                    message = message + error_message;
                }
            }
        }

        Self {
            code: StatusCode::UNPROCESSABLE_ENTITY,
            message,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "code": self.code.as_u16(),
            "message": self.message,
        });

        (self.code, Json(payload)).into_response()
    }
}
