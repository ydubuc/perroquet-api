use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EditReminderDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub frequency: Option<String>,
    pub trigger_at: Option<i64>,
}
