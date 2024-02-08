use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::app;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Reminder {
    pub id: sqlx::types::Uuid,
    pub body: String,
    pub created_at: i64,
}

impl Reminder {
    pub fn new(body: String) -> Self {
        return Self {
            id: Uuid::new_v4(),
            body: body.trim().to_string(),
            created_at: app::util::time::current_time_in_millis(),
        };
    }
}
