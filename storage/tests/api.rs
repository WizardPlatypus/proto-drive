use sqlx::PgPool;
use storage::api::Shared;
use tower::ServiceExt;
use uuid::uuid;

#[sqlx::test(migrations = "./migrations", fixtures("algernon"))]
async fn upload_success(pool: PgPool) {
    init_tracing();
    let token = storage::auth::jwt::issue(uuid!("331194d0-3c87-42ed-aab0-bac0fc637063"), "testing".as_bytes(), chrono::Duration::minutes(30)).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let shared = Shared { pool, jwt_secret: std::sync::Arc::from("testing".as_bytes()), root: dir.path().to_string_lossy().to_string() };
    let app = storage::app(shared);
    let body = axum::body::Body::from(
        concat!(
            "--BOUNDARY\r\n",
            "Content-Disposition: form-data; name=\"destination\"\r\n\r\n",
            "/docs\r\n",
            "--BOUNDARY\r\n",
            "Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n",
            "Content-Type: text/plain\r\n\r\n",
            "hello world\r\n",
            "--BOUNDARY--\r\n"
        )
    );
    let req = axum::http::Request::builder()
        .method("POST")
        .uri("/upload")
        .header("content-type", "multipart/form-data; boundary=BOUNDARY")
        .header(axum::http::header::AUTHORIZATION, format!("Bearer {}", &token))
        .body(body)
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::CREATED);
}

use once_cell::sync::OnceCell;
use tracing_subscriber::{fmt, EnvFilter};

static TRACING: OnceCell<()> = OnceCell::new();

pub fn init_tracing() {
    TRACING.get_or_init(|| {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"));

        fmt()
            .with_env_filter(filter)
            .with_test_writer() 
            .init();
    });
}
