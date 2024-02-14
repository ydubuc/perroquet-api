use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReminderDto {
    pub title: Option<String>,
    pub content: String,
    pub frequency: Option<String>,
    pub trigger_at: i64,
}
