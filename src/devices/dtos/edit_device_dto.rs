use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EditDeviceDto {
    pub messaging_token: Option<String>,
}
