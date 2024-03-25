use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateMemoDto {
    #[validate(custom = "super::validate_uuid")]
    pub id: String,
    #[validate(length(
        min = 1,
        max = 512,
        message = "title must be between 1 and 512 characters."
    ))]
    pub title: String,
    #[validate(length(
        min = 1,
        max = 65535,
        message = "description must be between 1 and 65535 characters."
    ))]
    pub description: Option<String>,
    pub priority: i16,
    pub visibility: i16,
    pub frequency: Option<String>,
    pub trigger_at: i64,
}
