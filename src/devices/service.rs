use axum::http::StatusCode;
use sqlx::Postgres;
use uuid::Uuid;

use crate::{
    app::{
        self,
        models::{api_error::ApiError, app_state::AppState},
        util::time,
    },
    auth::models::access_token_claims::AccessTokenClaims,
    users::models::user::User,
};

use super::{
    dtos::{edit_device_dto::EditDeviceDto, get_devices_filter_dto::GetDevicesFilterDto},
    models::device::Device,
};

pub async fn create_device(user: &User, state: &AppState) -> Result<Device, ApiError> {
    let device = Device::new(user);

    let sqlx_result = sqlx::query(
        "
        INSERT INTO devices (
            id, user_id, refresh_token, messaging_token,
            refreshed_at, updated_at, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ",
    )
    .bind(&device.id)
    .bind(&device.user_id)
    .bind(&device.refresh_token)
    .bind(&device.messaging_token)
    .bind(&device.refreshed_at)
    .bind(&device.updated_at)
    .bind(&device.created_at)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(_) => Ok(device),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create device.",
            ))
        }
    }
}

pub async fn get_devices(
    dto: &GetDevicesFilterDto,
    claims: Option<&AccessTokenClaims>,
    state: &AppState,
) -> Result<Vec<Device>, ApiError> {
    // SQL
    let mut query = "SELECT * FROM devices WHERE true".to_string();

    let mut sort_field = "refreshed_at".to_string();
    let mut sort_order = "DESC".to_string();
    let mut page_limit: u8 = 80;

    let mut index: u8 = 0;

    if claims.is_some() {
        index += 1;
        query.push_str(&format!(" AND user_id = ${}", index));
    }
    if dto.id.is_some() {
        index += 1;
        query.push_str(&format!(" AND id = ${}", index));
    }

    // SQL SORT
    if let Some(sort) = &dto.sort {
        match app::util::dto::get_sort_params(sort, None) {
            Ok(sort_params) => {
                sort_field = sort_params.field;
                sort_order = sort_params.order;
            }
            Err(e) => return Err(e),
        }

        if let Some(cursor) = &dto.cursor {
            match app::util::dto::get_cursor(&cursor) {
                Ok(cursor) => {
                    let carrot_sign = match sort_order.as_ref() {
                        "DESC" => "<",
                        _ => ">",
                    };
                    query.push_str(&format!(
                        " AND ({}, id) {} ({}, '{}')",
                        sort_field, carrot_sign, cursor.value, cursor.id
                    ))
                }
                Err(e) => return Err(e),
            }
        }
    }
    query.push_str(&format!(
        " ORDER BY {} {}, id {}",
        sort_field, sort_order, sort_order
    ));
    if let Some(limit) = dto.limit {
        page_limit = limit;
    }
    query.push_str(&format!(" LIMIT {}", page_limit));

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Device>(&query);

    if let Some(claims) = claims {
        sqlx = sqlx.bind(&claims.id);
    }
    if let Some(id) = &dto.id {
        sqlx = sqlx.bind(id)
    }

    let sqlx_result = sqlx.fetch_all(&state.pool).await;

    match sqlx_result {
        Ok(devices) => Ok(devices),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get devices.",
            ))
        }
    }
}

pub async fn refresh_device(refresh_token: &str, state: &AppState) -> Result<Device, ApiError> {
    let new_refresh_token = Uuid::new_v4().to_string();
    let current_time = time::current_time_in_millis();

    let sqlx_result = sqlx::query_as::<Postgres, Device>(
        "
        UPDATE devices SET refresh_token = $1, refreshed_at = $2, updated_at = $3
        WHERE refresh_token = $4 RETURNING *
        ",
    )
    .bind(&new_refresh_token)
    .bind(&current_time)
    .bind(&current_time)
    .bind(&refresh_token)
    .fetch_optional(&state.pool)
    .await;

    match sqlx_result {
        Ok(device) => match device {
            Some(device) => Ok(device),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "Device not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to refresh device.",
            ))
        }
    }
}

pub async fn edit_device(
    id: &str,
    dto: &EditDeviceDto,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<Device, ApiError> {
    // SQL
    let mut query = "UPDATE devices SET ".to_string();

    let mut index: u8 = 0;

    if dto.messaging_token.is_some() {
        index += 1;
        query.push_str(&format!("messaging_token = ${}, ", index));
    }

    index += 1;
    query.push_str(&format!("updated_at = ${} ", index));
    index += 1;
    query.push_str(&format!("WHERE id = ${} ", index));
    index += 1;
    query.push_str(&format!("AND user_id = ${} ", index));
    query.push_str(&format!("RETURNING *"));

    // SQLX
    let mut sqlx = sqlx::query_as::<Postgres, Device>(&query);

    if let Some(messaging_token) = &dto.messaging_token {
        sqlx = sqlx.bind(messaging_token);
    }
    sqlx = sqlx.bind(time::current_time_in_millis());
    sqlx = sqlx.bind(id);
    sqlx = sqlx.bind(&claims.id);

    let sqlx_result = sqlx.fetch_optional(&state.pool).await;

    match sqlx_result {
        Ok(device) => match device {
            Some(device) => Ok(device),
            None => Err(ApiError::new(StatusCode::NOT_FOUND, "Device not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to edit device.",
            ))
        }
    }
}

pub async fn delete_device(
    id: &str,
    claims: &AccessTokenClaims,
    state: &AppState,
) -> Result<(), ApiError> {
    let sqlx_result = sqlx::query(
        "
        DELETE FROM devices
        WHERE id = $1 AND user_id = $2
        ",
    )
    .bind(id)
    .bind(&claims.id)
    .execute(&state.pool)
    .await;

    match sqlx_result {
        Ok(result) => match result.rows_affected() > 0 {
            true => Ok(()),
            false => Err(ApiError::new(StatusCode::NOT_FOUND, "Device not found.")),
        },
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete device.",
            ))
        }
    }
}
