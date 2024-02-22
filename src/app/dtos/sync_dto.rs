use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::reminders::dtos::get_reminders_filter_dto::GetRemindersFilterDto;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SyncDto {
    pub user: Option<bool>,
    pub reminders: Option<GetRemindersFilterDto>,
}
