use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessInfo {
    pub access_token: String,
    pub refresh_token: String,
    pub device_id: String,
}
