use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    app, auth::models::claims::Claims, reminders::dtos::create_reminder_dto::CreateReminderDto,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Reminder {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub title: Option<String>,
    pub content: String,
    pub frequency: Option<String>,
    pub trigger_at: i64,
    pub updated_at: i64,
    pub created_at: i64,
}

impl Reminder {
    pub fn new(dto: &CreateReminderDto, claims: &Claims) -> Self {
        let current_time = app::util::time::current_time_in_millis();

        Self {
            id: Uuid::new_v4(),
            user_id: Uuid::from_str(&claims.id).unwrap(),
            title: dto.title.clone(),
            content: dto.content.trim().to_string(),
            frequency: dto.frequency.clone(),
            trigger_at: dto.trigger_at,
            updated_at: current_time,
            created_at: current_time,
        }
    }
}
