use std::{env, time::Duration};

use app::envy::Envy;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod reminders;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub envy: Envy,
}

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

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&envy.database_url)
        .await
        .expect("database connection failed");

    tracing::info!("connected to database");

    let app_state = AppState { db_pool, envy };

    // app
    let app = Router::new()
        .route("/", get(app::controller::get_root))
        .route("/reminders", post(reminders::controller::create_reminder))
        .route("/reminders", get(reminders::controller::get_reminders))
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
