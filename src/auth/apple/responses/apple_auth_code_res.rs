use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    app::models::app_error::AppError,
    auth::apple::models::{id_token_claims::IdTokenClaims, public_key::PublicKey},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleAuthCodeResponse {
    #[serde(rename(deserialize = "id_token"))]
    pub id_token: String,
    #[serde(rename(deserialize = "access_token"))]
    pub access_token: String,
    #[serde(rename(deserialize = "refresh_token"))]
    pub refresh_token: String,
    #[serde(rename(deserialize = "token_type"))]
    pub token_type: String,
    #[serde(rename(deserialize = "expires_in"))]
    pub expires_in: u64,
}

impl AppleAuthCodeResponse {
    pub fn decode_id_token(
        &self,
        public_keys: &Vec<PublicKey>,
        aud: &str,
    ) -> Result<IdTokenClaims, AppError> {
        let Ok(id_token_header) = jsonwebtoken::decode_header(&self.id_token) else {
            return Err(AppError::new("failed to decode id_token header"));
        };

        let Some(kid) = id_token_header.kid else {
            return Err(AppError::new("id_token_header has no kid"));
        };

        let public_keys_filter: Vec<&PublicKey> =
            public_keys.iter().filter(|key| key.kid == kid).collect();
        let Some(public_key) = public_keys_filter.first() else {
            return Err(AppError::new("no public key matched id_token"));
        };

        let Ok(decoding_key) = DecodingKey::from_rsa_components(&public_key.n, &public_key.e)
        else {
            return Err(AppError::new("failed to create decoding key"));
        };

        let mut validation = Validation::new(id_token_header.alg);
        validation.set_audience(&[aud]);

        match jsonwebtoken::decode::<IdTokenClaims>(&self.id_token, &decoding_key, &validation) {
            Ok(data) => Ok(data.claims),
            Err(e) => {
                tracing::error!(%e);
                Err(AppError::new("failed to decode id_token"))
            }
        }
    }
}
