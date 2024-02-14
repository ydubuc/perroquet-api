use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessInfo {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub device_id: Option<String>,
}
