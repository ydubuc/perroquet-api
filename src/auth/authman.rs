use std::sync::Arc;

use tokio::sync::RwLock;

use crate::app::fcm::client::FcmClient;

use super::apple::client::AppleAuthClient;

#[derive(Debug, Clone)]
pub struct AuthMan {
    apple_client: Arc<RwLock<AppleAuthClient>>,
    fcm_client: Arc<RwLock<FcmClient>>,
}

impl AuthMan {
    pub fn new(
        apple_client: Arc<RwLock<AppleAuthClient>>,
        fcm_client: Arc<RwLock<FcmClient>>,
    ) -> Self {
        Self {
            apple_client,
            fcm_client,
        }
    }

    pub async fn apple_client(
        &self,
        http_client: &reqwest::Client,
    ) -> Arc<RwLock<AppleAuthClient>> {
        let readable_apple_client = self.apple_client.read().await;
        if readable_apple_client.expired() {
            drop(readable_apple_client);
            let mut writable_apple_client = self.apple_client.write().await;
            let _ = writable_apple_client.login(http_client).await;
        }

        self.apple_client.clone()
    }

    pub async fn fcm_client(&self, http_client: &reqwest::Client) -> Arc<RwLock<FcmClient>> {
        let readable_fcm_client = self.fcm_client.read().await;
        if readable_fcm_client.expired() {
            drop(readable_fcm_client);
            let mut writeable_fcm_client = self.fcm_client.write().await;
            let _ = writeable_fcm_client.login(http_client).await;
        }

        self.fcm_client.clone()
    }
}
