use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    app, auth::models::access_token_claims::AccessTokenClaims,
    memos::dtos::create_memo_dto::CreateMemoDto,
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Memo {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    pub status: String,
    pub visibility: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    pub trigger_at: i64,
    pub updated_at: i64,
    pub created_at: i64,
}

impl Memo {
    pub fn new(dto: &CreateMemoDto, claims: &AccessTokenClaims) -> Self {
        let current_time = app::util::time::current_time_in_millis();

        Self {
            id: Uuid::from_str(&dto.id).unwrap(),
            user_id: Uuid::from_str(&claims.id).unwrap(),
            title: dto.title.trim().to_string(),
            description: match &dto.description {
                Some(description) => Some(description.trim().to_string()),
                None => None,
            },
            priority: dto.priority.clone(),
            status: "pending".to_string(),
            visibility: dto.visibility,
            frequency: dto.frequency.clone(),
            trigger_at: dto.trigger_at,
            updated_at: current_time,
            created_at: current_time,
        }
    }
}
