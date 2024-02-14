use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetRemindersFilterDto {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u8>,
}
