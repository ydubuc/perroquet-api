use std::{collections::HashMap, time::Instant};

use axum::http::StatusCode;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::app::{
    self,
    models::{api_error::ApiError, app_error::AppError},
};

use super::{
    config::Config,
    models::{client_claims::ClientClaims, public_key::PublicKey},
    structs::{
        apple_auth_code_res::AppleAuthCodeResponse, apple_public_keys_res::ApplePublicKeysResponse,
    },
};

#[derive(Debug, Clone)]
pub struct AppleAuthClient {
    pub config: Config,
    client_secret: String,
    pub public_keys: Vec<PublicKey>,
    refreshed_at: Instant,
}

impl AppleAuthClient {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client_secret: String::new(),
            public_keys: Vec::new(),
            refreshed_at: Instant::now(),
        }
    }

    pub fn expired(&self) -> bool {
        self.refreshed_at.elapsed().as_secs() > 3600
    }

    pub async fn login(&mut self, http_client: &reqwest::Client) -> Result<(), AppError> {
        let current_time_in_secs = app::util::time::current_time_in_secs();
        let claims = serde_json::json!(ClientClaims {
            iss: self.config.team_id.to_string(),
            sub: self.config.client_id.to_string(),
            aud: "https://appleid.apple.com".to_string(),
            iat: current_time_in_secs,
            exp: current_time_in_secs + 3600,
        });
        let Ok(encoding_key) = EncodingKey::from_ec_pem(self.config.private_key.as_bytes()) else {
            return Err(AppError::new(
                "auth::util::apple::client failed to encode private key",
            ));
        };
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.config.key_id.to_string());
        let Ok(client_secret) = encode(&header, &claims, &encoding_key) else {
            return Err(AppError::new(
                "auth::util::apple::client failed to encode claims",
            ));
        };

        let result = http_client
            .get("https://appleid.apple.com/auth/keys")
            .send()
            .await;

        let Ok(response) = result else {
            return Err(AppError::new("failed to get apple public keys"));
        };
        let Ok(text) = response.text().await else {
            return Err(AppError::new("failed to get response text"));
        };
        let Ok(apple_public_keys_res) = serde_json::from_str::<ApplePublicKeysResponse>(&text)
        else {
            return Err(AppError::new("failed to decode public keys from text"));
        };

        self.client_secret = client_secret;
        self.public_keys = apple_public_keys_res.keys;
        self.refreshed_at = Instant::now();

        Ok(())
    }

    pub async fn validate_auth_code(
        &self,
        auth_code: &str,
        http_client: &reqwest::Client,
    ) -> Result<AppleAuthCodeResponse, ApiError> {
        let mut form = HashMap::new();
        form.insert("client_id", self.config.client_id.to_string());
        form.insert("client_secret", self.client_secret.to_string());
        form.insert("code", auth_code.to_string());
        form.insert("grant_type", "authorization_code".to_string());

        let result = http_client
            .post("https://appleid.apple.com/auth/token")
            .form(&form)
            .send()
            .await;

        match result {
            Ok(res) => match res.text().await {
                Ok(text) => match serde_json::from_str(&text) {
                    Ok(res) => Ok(res),
                    Err(_) => {
                        tracing::error!(%text);
                        Err(ApiError::new(
                            StatusCode::UNAUTHORIZED,
                            "Failed to authorize Apple auth code.",
                        ))
                    }
                },
                Err(e) => {
                    tracing::error!(%e);
                    Err(ApiError::new(
                        StatusCode::UNAUTHORIZED,
                        "Failed to authorize Apple auth code.",
                    ))
                }
            },
            Err(e) => {
                tracing::error!(%e);
                Err(ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to request Apple authorization.",
                ))
            }
        }
    }
}
