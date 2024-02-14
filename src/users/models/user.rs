use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{app::util::time, users::util};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: sqlx::types::Uuid,
    pub id_apple: Option<String>,
    pub username: String,
    pub username_key: String,
    pub email: String,
    pub email_key: String,
    pub password: String,
    pub displayname: String,
    pub avatar_url: Option<String>,
    pub updated_at: i64,
    pub created_at: i64,
}

impl User {
    pub fn new(
        username: &Option<String>,
        email: &str,
        password_hash: &str,
        id_apple: &Option<String>,
    ) -> Self {
        let current_time = time::current_time_in_millis();
        let username = username.clone().unwrap_or(util::username::new());

        Self {
            id: Uuid::new_v4(),
            id_apple: match id_apple {
                Some(id_apple) => Some(id_apple.to_string()),
                None => None,
            },
            username: username.to_string(),
            username_key: username.to_lowercase(),
            email: email.to_string(),
            email_key: email.to_lowercase(),
            password: password_hash.to_string(),
            displayname: username,
            avatar_url: None,
            updated_at: current_time,
            created_at: current_time,
        }
    }
}
