use axum::http::StatusCode;

use crate::app::models::api_error::ApiError;

pub struct SortParams {
    pub field: String,
    pub order: String,
}

pub fn get_sort_params(
    sort_string: &str,
    sortable_fields: Option<Vec<&'static str>>,
) -> Result<SortParams, ApiError> {
    let sort_params: Vec<&str> = sort_string.split(",").collect();

    if sort_params.len() != 2 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Malformed sort query.",
        ));
    }

    if let Some(sortable_fields) = sortable_fields {
        if !sortable_fields.contains(&sort_params[0]) {
            return Err(ApiError::new(StatusCode::BAD_REQUEST, "Invalid sort field"));
        }
    }

    let order = match sort_params[1] {
        "ASC" => "ASC",
        "DESC" => "DESC",
        _ => {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "Malformed sort query.",
            ));
        }
    };

    return Ok(SortParams {
        field: sort_params[0].to_string(),
        order: order.to_string(),
    });
}

pub struct Cursor {
    pub value: String,
    pub id: String,
}

pub fn get_cursor(cursor_string: &str) -> Result<Cursor, ApiError> {
    let cursor_params: Vec<&str> = cursor_string.split(",").collect();

    if cursor_params.len() != 2 {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Malformed cursor."));
    }

    return Ok(Cursor {
        value: cursor_params[0].to_string(),
        id: cursor_params[1].to_string(),
    });
}
