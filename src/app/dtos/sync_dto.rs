use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::memos::dtos::get_memos_dto::GetMemosDto;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SyncDto {
    pub user: Option<bool>,
    pub memos: Option<GetMemosDto>,
}
