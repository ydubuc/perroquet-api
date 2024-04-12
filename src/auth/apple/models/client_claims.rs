use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientClaims {
    #[serde(rename(deserialize = "iss"))]
    pub iss: String,
    #[serde(rename(deserialize = "sub"))]
    pub sub: String,
    #[serde(rename(deserialize = "aud"))]
    pub aud: String,
    #[serde(rename(deserialize = "iat"))]
    pub iat: i64,
    #[serde(rename(deserialize = "exp"))]
    pub exp: i64,
}
