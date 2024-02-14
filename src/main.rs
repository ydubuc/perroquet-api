use std::{env, sync::Arc, time::Duration};

use auth::authman::AuthMan;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app::models::app_state::AppState,
    auth::apple::{self, client::AppleAuthClient},
};

mod app;
mod auth;
mod reminders;
mod users;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    // environment
    let app_env = env::var("APP_ENV").unwrap_or("development".to_string());
    let _ = dotenvy::from_filename(format!(".env.{}", app_env));
    let envy = match envy::from_env::<app::envy::Envy>() {
        Ok(config) => config,
        Err(e) => panic!("{:#?}", e),
    };

    // logging
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

    // properties
    let port = envy.port.to_owned().unwrap_or(3000);
    let http_client = reqwest::Client::new();

    let apple_config = apple::config::Config {
        team_id: envy.apple_team_id.to_owned(),
        client_id: envy.apple_client_id.to_owned(),
        key_id: envy.apple_key_id.to_owned(),
        private_key: envy.apple_private_key.to_owned(),
    };
    let mut apple_client = AppleAuthClient::new(apple_config);
    apple_client
        .login(&http_client)
        .await
        .expect("failed to login to apple_client");

    let authman = AuthMan::new(Arc::new(RwLock::new(apple_client)));

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&envy.database_url)
        .await
        .expect("database connection failed");
    tracing::info!("connected to database");

    let app_state = AppState {
        envy,
        http_client,
        authman,
        pool,
    };

    // app
    let app = Router::new()
        .route("/", get(app::controller::get_root))
        .route("/auth/signup", post(auth::controller::signup))
        .route("/auth/signin", post(auth::controller::signin))
        .route("/auth/signin/apple", post(auth::controller::signin_apple))
        .route("/reminders", post(reminders::controller::create_reminder))
        .route("/reminders", get(reminders::controller::get_reminders))
        .route("/reminders/:id", get(reminders::controller::get_reminder))
        .route(
            "/reminders/:id",
            patch(reminders::controller::edit_reminder),
        )
        .route(
            "/reminders/:id",
            delete(reminders::controller::delete_reminder),
        )
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap()
}
