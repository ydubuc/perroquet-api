use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RefreshAccessInfoDto {
    pub refresh_token: String,
}
