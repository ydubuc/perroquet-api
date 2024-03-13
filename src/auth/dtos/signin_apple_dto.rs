use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SigninAppleDto {
    pub auth_code: String,
    pub client: Option<String>,
}
