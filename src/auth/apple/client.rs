use std::{collections::HashMap, time::Instant};

use axum::http::StatusCode;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::app::{
    self,
    models::{api_error::ApiError, app_error::AppError},
};

use super::{
    models::{client_claims::ClientClaims, client_config::ClientConfig, public_key::PublicKey},
    responses::{
        apple_auth_code_res::AppleAuthCodeResponse, apple_public_keys_res::ApplePublicKeysResponse,
    },
};

#[derive(Debug, Clone)]
pub struct AppleAuthClient {
    pub config: ClientConfig,
    client_secret_ios: String,
    client_secret_android: String,
    client_secret_web: String,
    pub public_keys: Vec<PublicKey>,
    refreshed_at: Instant,
}

impl AppleAuthClient {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            client_secret_ios: String::new(),
            client_secret_android: String::new(),
            client_secret_web: String::new(),
            public_keys: Vec::new(),
            refreshed_at: Instant::now(),
        }
    }

    pub fn expired(&self) -> bool {
        self.refreshed_at.elapsed().as_secs() > 3600
    }

    pub async fn login(&mut self, http_client: &reqwest::Client) -> Result<(), AppError> {
        let client_secret_ios =
            Self::generate_client_secret(self.config.clone(), "ios".to_string())?;
        let client_secret_web =
            Self::generate_client_secret(self.config.clone(), "web".to_string())?;
        let client_secret_android =
            Self::generate_client_secret(self.config.clone(), "android".to_string())?;

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

        self.client_secret_ios = client_secret_ios;
        self.client_secret_android = client_secret_android;
        self.client_secret_web = client_secret_web;
        self.public_keys = apple_public_keys_res.keys;
        self.refreshed_at = Instant::now();

        Ok(())
    }

    fn generate_client_secret(
        config: ClientConfig,
        client_type: String,
    ) -> Result<String, AppError> {
        let client_id = match client_type.as_ref() {
            "ios" => config.client_id.clone(),
            "android" => format!("{}{}", config.client_id, ".Android"),
            "web" => format!("{}{}", config.client_id, ".Web"),
            _ => config.client_id.clone(),
        };

        let current_time_in_secs = app::util::time::current_time_in_secs();
        let claims = serde_json::json!(ClientClaims {
            iss: config.team_id.to_string(),
            sub: client_id,
            aud: "https://appleid.apple.com".to_string(),
            iat: current_time_in_secs,
            exp: current_time_in_secs + 3600,
        });
        let Ok(encoding_key) = EncodingKey::from_ec_pem(config.private_key.as_bytes()) else {
            return Err(AppError::new(
                "auth::util::apple::client failed to encode private key",
            ));
        };
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(config.key_id.to_string());
        let Ok(client_secret) = encode(&header, &claims, &encoding_key) else {
            return Err(AppError::new(
                "auth::util::apple::client failed to encode claims",
            ));
        };

        Ok(client_secret)
    }

    pub async fn validate_auth_code(
        &self,
        auth_code: &str,
        client_type: String,
        http_client: &reqwest::Client,
    ) -> Result<AppleAuthCodeResponse, ApiError> {
        let client_id = match client_type.as_ref() {
            "ios" => format!("{}", self.config.client_id),
            "android" => format!("{}.Android", self.config.client_id),
            "web" => format!("{}.Web", self.config.client_id),
            _ => self.config.client_id.clone(),
        };

        let client_secret = match client_type.as_ref() {
            "ios" => &self.client_secret_ios,
            "android" => &self.client_secret_android,
            "web" => &self.client_secret_web,
            _ => &self.client_secret_ios,
        };

        let mut form = HashMap::new();
        form.insert("client_id", client_id);
        form.insert("client_secret", client_secret.to_string());
        form.insert("code", auth_code.to_string());
        form.insert("grant_type", "authorization_code".to_string());

        if client_type == "web" {
            form.insert(
                "redirect_uri",
                "https://perroquet.beamcove.com/signin/apple".to_string(),
            );
        }

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
