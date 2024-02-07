use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Envy {
    pub app_env: String,
    pub port: Option<u16>,
}
