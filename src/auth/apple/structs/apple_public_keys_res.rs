use serde::Deserialize;

use crate::auth::apple::models::public_key::PublicKey;

#[derive(Debug, Deserialize)]
pub struct ApplePublicKeysResponse {
    pub keys: Vec<PublicKey>,
}
