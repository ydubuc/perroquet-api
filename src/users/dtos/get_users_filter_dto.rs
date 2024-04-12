use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetUsersFilterDto {
    pub id: Option<String>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub cursor: Option<String>,
    #[validate(range(max = 50, message = "limit must be equal or less than 50."))]
    pub limit: Option<u8>,
}
