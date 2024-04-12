use std::{collections::HashMap, time::Instant};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::{header, Body, StatusCode};
use serde_json::json;

use crate::app::{self, models::app_error::AppError};

use super::{
    models::{client_claims::ClientClaims, client_config::ClientConfig, fcm_message::FcmMessage},
    responses::fcm_oauth_res::FcmOAuthResponse,
};

#[derive(Clone, Debug)]
pub struct FcmClient {
    config: ClientConfig,
    authorization_token: String,
    refreshed_at: Instant,
}

impl FcmClient {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            authorization_token: String::new(),
            refreshed_at: Instant::now(),
        }
    }

    pub fn expired(&self) -> bool {
        self.refreshed_at.elapsed().as_secs() > 3600
    }

    pub async fn login(&mut self, http_client: &reqwest::Client) -> Result<(), AppError> {
        let current_time_in_secs = app::util::time::current_time_in_secs();
        let claims = serde_json::json!(ClientClaims {
            iss: self.config.client_email.to_string(),
            scope: "https://www.googleapis.com/auth/firebase.messaging".to_string(),
            aud: "https://www.googleapis.com/oauth2/v4/token".to_string(),
            iat: current_time_in_secs,
            exp: current_time_in_secs + 3600,
        });
        let Ok(encoding_key) = EncodingKey::from_rsa_pem(self.config.private_key.as_bytes()) else {
            return Err(AppError::new(
                "app::util::fcm::client failed to encode private key",
            ));
        };
        let header = Header::new(Algorithm::RS256);
        let Ok(token) = encode(&header, &claims, &encoding_key) else {
            return Err(AppError::new(
                "auth::util::apple::client failed to encode claims",
            ));
        };
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut body = HashMap::new();
        body.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
        body.insert("assertion", &token);

        let result = http_client
            .post("https://www.googleapis.com/oauth2/v4/token")
            .headers(headers)
            .json(&body)
            .send()
            .await;

        let Ok(response) = result else {
            return Err(AppError::new("failed to get fcm token"));
        };
        let Ok(text) = response.text().await else {
            return Err(AppError::new("failed to get response text"));
        };
        let Ok(fcm_oauth_res) = serde_json::from_str::<FcmOAuthResponse>(&text) else {
            return Err(AppError::new("failed to decode fcm oauth res from text"));
        };

        self.authorization_token = fcm_oauth_res.access_token;
        self.refreshed_at = Instant::now();

        Ok(())
    }

    pub async fn send(
        &self,
        message: FcmMessage,
        http_client: &reqwest::Client,
    ) -> Result<(), String> {
        let click_action = match message.click_action {
            Some(click_action) => click_action,
            None => "none".to_string(),
        };

        let fcm_message = json!({
            "message": {
                "token": message.token,
                "notification": {
                    "title": message.title,
                    "body": message.body,
                },
                "android": {
                    "notification": {
                        "click_action": click_action
                    }
                },
                "apns": {
                    "payload": {
                        "aps": {
                            "sound": "default",
                            "category": click_action
                        }
                    }
                }
            }
        });

        let Ok(payload) = serde_json::to_vec(&fcm_message) else {
            return Err(message.token);
        };

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Content-Length", (payload.len() as u64).into());
        headers.insert(
            "Authorization",
            ["Bearer ", &self.authorization_token]
                .concat()
                .parse()
                .unwrap(),
        );

        let result = http_client
            .post(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                self.config.project_name
            ))
            .headers(headers)
            .body(Body::from(payload))
            .send()
            .await;

        match result {
            Ok(res) => match res.status() {
                StatusCode::OK => return Ok(()),
                _ => {
                    tracing::error!("{:?}", res.text().await);
                    return Err(message.token);
                }
            },
            Err(e) => {
                tracing::error!(%e);
                return Err(message.token);
            }
        }
    }
}
