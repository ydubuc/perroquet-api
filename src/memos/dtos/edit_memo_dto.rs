use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EditMemoDto {
    #[validate(length(
        min = 1,
        max = 512,
        message = "title must be between 1 and 512 characters."
    ))]
    pub title: Option<String>,
    #[validate(length(
        min = 1,
        max = 65535,
        message = "description must be between 1 and 65535 characters."
    ))]
    pub description: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub visibility: Option<i16>,
    pub frequency: Option<String>,
    pub trigger_at: Option<i64>,
}
