use std::{sync::Arc, time::Duration};

use axum::{Router, routing::post};
use sqlx::postgres::PgPoolOptions;
use storage::api;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let db_connection_string =
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
    let jwt_secret = Arc::from(
        std::env::var("JWT_SECRET")
            .expect("Missing JWT_SECRET environment variable")
            .as_bytes(),
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_string)
        .await
        .expect("Failed to connect to the database");
    let shared = storage::api::Shared { pool, jwt_secret };
    let app = Router::new()
        .route("/auth/register", post(api::register))
        .route("/auth/login", post(api::login))
        .route("/upload", post(api::upload_file))
        .route("/download/:file_id", post(api::download_file))
        .with_state(shared);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
