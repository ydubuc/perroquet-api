use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{app::util::time, users::util};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: sqlx::types::Uuid,
    #[serde(skip_serializing)]
    pub id_apple: Option<String>,
    pub username: String,
    #[serde(skip_serializing)]
    pub username_key: String,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub email_key: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub displayname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub updated_at: i64,
    pub created_at: i64,
}

impl User {
    pub fn new(
        username: &Option<String>,
        email: &str,
        password_hash: &Option<String>,
        id_apple: &Option<String>,
    ) -> Self {
        let current_time = time::current_time_in_millis();
        let username = username.clone().unwrap_or(util::username::new());

        Self {
            id: Uuid::new_v4(),
            id_apple: id_apple.clone(),
            username: username.to_string(),
            username_key: username.to_lowercase(),
            email: email.to_string(),
            email_key: email.to_lowercase(),
            password: password_hash.clone(),
            displayname: username,
            avatar_url: None,
            updated_at: current_time,
            created_at: current_time,
        }
    }
}
