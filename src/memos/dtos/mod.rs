use std::borrow::Cow;

use uuid::Uuid;
use validator::ValidationError;

pub mod create_memo_dto;
pub mod edit_memo_dto;
pub mod get_memos_dto;

pub fn validate_uuid(value: &str) -> Result<(), ValidationError> {
    match Uuid::parse_str(value).is_ok() {
        true => Ok(()),
        false => {
            let mut error = ValidationError::new("invalid_uuid");
            error.message = Some(Cow::from("id must be a valid uuid."));
            Err(error)
        }
    }
}
