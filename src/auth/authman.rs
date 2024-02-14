use std::sync::Arc;

use tokio::sync::RwLock;

use super::apple::client::AppleAuthClient;

#[derive(Debug, Clone)]
pub struct AuthMan {
    apple_client: Arc<RwLock<AppleAuthClient>>,
}

impl AuthMan {
    pub fn new(apple_client: Arc<RwLock<AppleAuthClient>>) -> Self {
        Self { apple_client }
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

        return self.apple_client.clone();
    }
}
