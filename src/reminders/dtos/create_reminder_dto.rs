use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateReminderDto {
    #[validate(length(
        min = 1,
        max = 512,
        message = "title must be between 1 and 512 characters."
    ))]
    pub title: Option<String>,
    #[validate(length(
        min = 1,
        max = 65535,
        message = "content must be between 1 and 65535 characters."
    ))]
    pub content: String,
    pub frequency: Option<String>,
    pub trigger_at: i64,
}
