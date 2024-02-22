use serde::{Deserialize, Serialize};

use crate::{reminders::models::reminder::Reminder, users::models::user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminder>>,
}
