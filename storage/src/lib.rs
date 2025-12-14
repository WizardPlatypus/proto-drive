pub mod api;
pub mod auth;
pub mod db;

use axum::{Router, routing::post};

use crate::api::Shared;

pub fn app(shared: Shared) -> Router {
    Router::new()
        .route("/auth/register", post(api::register))
        .route("/auth/login", post(api::login))
        .route("/upload", post(api::upload_file))
        .route("/download/:file_id", post(api::download_file))
        .with_state(shared)
}
