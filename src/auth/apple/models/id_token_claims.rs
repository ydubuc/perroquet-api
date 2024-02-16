use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdTokenClaims {
    #[serde(rename(deserialize = "email"))]
    pub email: String,
    #[serde(rename(deserialize = "email_verified"))]
    pub email_verified: bool,
    #[serde(rename(deserialize = "is_private_email"))]
    pub is_private_email: bool,
    #[serde(rename(deserialize = "at_hash"))]
    pub at_hash: String,
    #[serde(rename(deserialize = "auth_time"))]
    pub auth_time: u64,
    #[serde(rename(deserialize = "nonce_supported"))]
    pub nonce_supported: bool,
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
