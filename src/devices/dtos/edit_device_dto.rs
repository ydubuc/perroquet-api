use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EditDeviceDto {
    pub messaging_token: Option<String>,
}
