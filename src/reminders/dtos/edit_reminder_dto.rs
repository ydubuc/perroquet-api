use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EditReminderDto {
    #[validate(length(
        min = 1,
        max = 512,
        message = "title must be between 1 and 512 characters."
    ))]
    pub title: Option<String>,
    #[validate(length(
        min = 1,
        max = 65535,
        message = "body must be between 1 and 65535 characters."
    ))]
    pub body: Option<String>,
    pub frequency: Option<String>,
    pub visibility: Option<i16>,
    pub trigger_at: Option<i64>,
}
