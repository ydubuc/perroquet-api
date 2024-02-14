use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    app::models::app_error::AppError,
    auth::apple::models::{id_token_claims::IdTokenClaims, public_key::PublicKey},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleAuthCodeResponse {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl AppleAuthCodeResponse {
    pub fn decode_id_token(&self, public_keys: &Vec<PublicKey>) -> Result<IdTokenClaims, AppError> {
        let Ok(id_token_header) = jsonwebtoken::decode_header(&self.id_token) else {
            return Err(AppError::new("failed to decode id_token header"));
        };
        tracing::debug!("header: {:?}", id_token_header);

        let Some(kid) = id_token_header.kid else {
            return Err(AppError::new("id_token_header has no kid"));
        };
        let public_keys_filter: Vec<&PublicKey> =
            public_keys.iter().filter(|key| key.kid == kid).collect();
        let Some(public_key) = public_keys_filter.first() else {
            return Err(AppError::new("no public key matched id_token"));
        };

        tracing::info!("public key: {:?}", public_key);

        let decoding_key =
            DecodingKey::from_rsa_components(&public_key.n, &public_key.e).expect("test");
        // let decoding_key =
        //     DecodingKey::from_rsa_raw_components(public_key.n.as_bytes(), public_key.e.as_bytes());
        let validation = Validation::new(Algorithm::RS256);
        match jsonwebtoken::decode::<IdTokenClaims>(&self.id_token, &decoding_key, &validation) {
            Ok(data) => Ok(data.claims),
            Err(e) => {
                tracing::error!(%e);
                Err(AppError::new("failed to decode id_token"))
            }
        }
    }
}
