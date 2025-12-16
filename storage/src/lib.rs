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
        .route("/folder", get(api::find_files))
        .route("/folder/{file_id}", get(api::get_folder))
        .route("/config", get(api::get_config))
        .route("/config", put(api::put_config))
        .with_state(shared)
        .layer(
            tower::ServiceBuilder::new().layer(
                tower_http::trace::TraceLayer::new_for_http()
                    .make_span_with(|request: &axum::extract::Request| {
                        tracing::info_span!(
                            "http_request",
                            method = ?request.method(),
                            uri = %request.uri(),
                        )
                    })
                    .on_request(|_request: &axum::extract::Request, _span: &tracing::Span| {
                        tracing::info!("Starting request");
                    })
                    .on_response(
                        |response: &axum::response::Response,
                         latency: std::time::Duration,
                         _span: &tracing::Span| {
                            tracing::info!(
                                status = %response.status(),
                                latency = ?latency,
                                "Request finished"
                            );
                        },
                    )
                    .on_body_chunk(tower_http::trace::DefaultOnBodyChunk::new())
                    .on_eos(tower_http::trace::DefaultOnEos::new())
                    .on_failure(tower_http::trace::DefaultOnFailure::new()),
            ),
        )
}
