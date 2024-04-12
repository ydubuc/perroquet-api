use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FcmOAuthResponse {
    #[serde(rename(deserialize = "access_token"))]
    pub access_token: String,
}
