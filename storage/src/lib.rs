pub mod api;
pub mod auth;
pub mod db;

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::api::Shared;

pub fn app(shared: Shared) -> Router {
    Router::new()
        .route("/auth/register", post(api::register))
        .route("/auth/login", post(api::login))
        .route("/upload", post(api::upload_file))
        .route("/download/{file_id}", get(api::download_file))
        .route("/files", get(api::get_config))
        .route("/config", get(api::get_config))
        .route("/config", put(api::put_config))
        .with_state(shared)
}
