use std::env;

use axum::{routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::service::handler;

mod app;

#[tokio::main]
async fn main() {
    let app_env = env::var("APP_ENV").unwrap_or("development".to_string());
    let _ = dotenvy::from_filename(format!(".env.{}", app_env));
    let envy = match envy::from_env::<app::envy::Envy>() {
        Ok(config) => config,
        Err(e) => panic!("{:#?}", e),
    };

    let log_level = match app_env.as_ref() {
        "production" => "info",
        _ => "debug",
    };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("perroquet_api={}", log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(handler));

    let addr = format!("0.0.0.0:{}", envy.port.unwrap_or(3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
