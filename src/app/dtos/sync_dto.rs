use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SyncDto {
    pub user: bool,
    pub reminders: bool,
}
