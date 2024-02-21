use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Envy {
    pub app_env: String,
    pub port: Option<u16>,

    pub database_url: String,

    pub jwt_secret: String,

    pub apple_team_id: String,
    pub apple_client_id: String,
    pub apple_key_id: String,
    pub apple_private_key: String,

    pub fcm_project_name: String,
    pub fcm_client_email: String,
    pub fcm_private_key: String,
}
