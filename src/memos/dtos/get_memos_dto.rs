use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetMemosDto {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub search: Option<String>,
    pub priority: Option<i16>,
    pub status: Option<String>,
    pub visibility: Option<i16>,
    pub sort: Option<String>,
    pub cursor: Option<String>,
    #[validate(range(max = 100, message = "limit must be equal or less than 100."))]
    pub limit: Option<u8>,
}
