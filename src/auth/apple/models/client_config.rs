#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub team_id: String,
    pub client_id_ios: String,
    pub client_id_android: String,
    pub client_id_web: String,
    pub key_id: String,
    pub private_key: String,
}
