use std::{env, sync::Arc, time::Duration};

use auth::authman::AuthMan;
use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{get, patch, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app::{
        fcm::{self, client::FcmClient},
        models::app_state::AppState,
    },
    auth::apple::{self, client::AppleAuthClient},
};

mod app;
mod auth;
mod devices;
mod mail;
mod memos;
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
    let port = envy.port.to_owned().unwrap_or(3001);
    let cors = CorsLayer::new()
        // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin(
            "https://perroquet.beamcove.com"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([Method::POST, Method::GET, Method::PATCH, Method::DELETE]);
    let http_client = reqwest::Client::new();

    let apple_config = apple::models::client_config::ClientConfig {
        team_id: envy.apple_team_id.to_owned(),
        client_id_ios: envy.apple_client_id.to_owned(),
        client_id_android: format!("{}.Android", envy.apple_client_id),
        client_id_web: format!("{}.Web", envy.apple_client_id),
        key_id: envy.apple_key_id.to_owned(),
        private_key: envy.apple_private_key.to_owned(),
    };
    let mut apple_client = AppleAuthClient::new(apple_config);
    apple_client
        .login(&http_client)
        .await
        .expect("failed to login to apple_client");

    let fcm_config = fcm::models::client_config::ClientConfig {
        project_name: envy.fcm_project_name.to_owned(),
        client_email: envy.fcm_client_email.to_owned(),
        private_key: envy.fcm_private_key.to_owned(),
    };
    let mut fcm_client = FcmClient::new(fcm_config);
    fcm_client
        .login(&http_client)
        .await
        .expect("failed to login to fcm_client");

    let authman = AuthMan::new(
        Arc::new(RwLock::new(apple_client)),
        Arc::new(RwLock::new(fcm_client)),
    );

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
        .route("/v1/", get(app::controller::get_root))
        .route("/v1/auth/signup", post(auth::controller::signup))
        .route("/v1/auth/signin", post(auth::controller::signin))
        .route(
            "/v1/auth/signin/apple",
            post(auth::controller::signin_apple),
        )
        .route("/v1/auth/refresh", post(auth::controller::refresh))
        .route("/v1/auth/signout", post(auth::controller::signout))
        .route(
            "/v1/auth/email",
            post(auth::controller::request_email_update),
        )
        .route(
            "/v1/auth/email",
            patch(auth::controller::process_email_update),
        )
        .route(
            "/v1/auth/password",
            post(auth::controller::request_password_update),
        )
        .route("/v1/auth/password", patch(auth::controller::edit_password))
        .route("/v1/devices", get(devices::controller::get_devices))
        .route("/v1/devices/:id", patch(devices::controller::edit_device))
        .route("/v1/users", get(users::controller::get_users))
        .route("/v1/users/me", get(users::controller::get_me))
        .route("/v1/memos", post(memos::controller::create_memo))
        .route("/v1/memos", get(memos::controller::get_memos))
        .route("/v1/memos/:id", get(memos::controller::get_memo))
        .route("/v1/memos/:id", patch(memos::controller::edit_memo))
        .layer(cors)
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap()
}
