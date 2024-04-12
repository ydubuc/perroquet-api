use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FcmMessage {
    pub token: String,
    pub title: String,
    pub body: String,
    pub click_action: Option<String>,
}
