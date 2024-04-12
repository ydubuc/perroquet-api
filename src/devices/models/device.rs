use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{app, users::models::user::User};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    #[serde(skip_serializing)]
    pub refresh_token: String,
    #[serde(skip_serializing)]
    pub messaging_token: Option<String>,
    pub refreshed_at: i64,
    pub updated_at: i64,
    pub created_at: i64,
}

impl Device {
    pub fn new(user: &User) -> Self {
        let current_time = app::util::time::current_time_in_millis();

        Self {
            id: Uuid::new_v4(),
            user_id: user.id,
            refresh_token: Uuid::new_v4().to_string(),
            messaging_token: None,
            refreshed_at: current_time,
            updated_at: current_time,
            created_at: current_time,
        }
    }
}
