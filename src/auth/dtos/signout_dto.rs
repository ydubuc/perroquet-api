use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignoutDto {
    pub device_id: String,
}
