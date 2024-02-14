use std::borrow::Cow;

use sqlx::error::DatabaseError;

#[non_exhaustive]
pub struct SqlStateCodes;

impl SqlStateCodes {
    pub const UNIQUE_VIOLATION: &'static str = "23505";
}

pub fn extract_db_err_code(db_err: &dyn DatabaseError) -> Option<String> {
    match db_err.code() {
        Some(code) => match code {
            Cow::Borrowed(val) => Some(val.to_owned()),
            Cow::Owned(val) => Some(val),
        },
        None => None,
    }
}
