use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdTokenClaims {
    pub email: String,
    pub email_verified: bool,
    pub is_private_email: bool,
    pub at_hash: String,
    pub auth_time: u64,
    pub nonce_supported: bool,
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
}

// {
//   "iss": "https://appleid.apple.com",
//   "aud": "com.beamcove.Perroquet",
//   "exp": 1707940846,
//   "iat": 1707854446,
//   "sub": "001690.1f938437e59a41f6aaabc78f10f6237c.2027",
//   "at_hash": "uRVYx9H0ZRQZNHtNrIM3MA",
//   "email": "mkq6mvmwhd@privaterelay.appleid.com",
//   "email_verified": true,
//   "is_private_email": true,
//   "auth_time": 1707854434,
//   "nonce_supported": true
// }
