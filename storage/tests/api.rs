use once_cell::sync::OnceCell;
use sqlx::PgPool;
use storage::api::Shared;
use tower::ServiceExt;
use tracing_subscriber::{EnvFilter, fmt};
use uuid::uuid;

static TRACING: OnceCell<()> = OnceCell::new();

pub fn init_tracing() {
    TRACING.get_or_init(|| {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

        fmt().with_env_filter(filter).with_test_writer().init();
    });
}

#[sqlx::test(migrations = "./migrations", fixtures("algernon"))]
async fn upload_success(pool: PgPool) {
    init_tracing();
    let token = storage::auth::jwt::issue(
        uuid!("331194d0-3c87-42ed-aab0-bac0fc637063"),
        "testing".as_bytes(),
        chrono::Duration::minutes(30),
    )
    .unwrap();
    let dir = tempfile::tempdir().unwrap();
    let shared = Shared {
        pool,
        jwt_secret: std::sync::Arc::from("testing".as_bytes()),
        root: dir.path().to_path_buf(),
    };
    let app = storage::app(shared);
    let body = axum::body::Body::from(concat!(
        "--BOUNDARY\r\n",
        "Content-Disposition: form-data; name=\"destination\"\r\n\r\n",
        "/docs\r\n",
        "--BOUNDARY\r\n",
        "Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n",
        "Content-Type: text/plain\r\n\r\n",
        "hello world\r\n",
        "--BOUNDARY--\r\n"
    ));
    let req = axum::http::Request::builder()
        .method("POST")
        .uri("/upload")
        .header("content-type", "multipart/form-data; boundary=BOUNDARY")
        .header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", &token),
        )
        .body(body)
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::CREATED);
}

#[sqlx::test(migrations = "./migrations", fixtures("algernon", "hello_world"))]
async fn download_success(pool: PgPool) {
    init_tracing();
    let token = storage::auth::jwt::issue(
        uuid!("331194d0-3c87-42ed-aab0-bac0fc637063"),
        "testing".as_bytes(),
        chrono::Duration::minutes(30),
    )
    .unwrap();
    let dir = tempfile::tempdir().unwrap();
    let shared = Shared {
        pool,
        jwt_secret: std::sync::Arc::from("testing".as_bytes()),
        root: dir.path().to_path_buf(),
    };
    tracing::debug!("root is {}", dir.path().to_string_lossy());
    let file_id = uuid!("7b798b53-5d49-404d-991f-ca92f74364e7");
    let user_id = uuid!("331194d0-3c87-42ed-aab0-bac0fc637063");
    let path = shared.root.join("storage").join(user_id.to_string()).join(file_id.to_string());
    std::fs::create_dir_all(path.parent().unwrap())
        .expect("Failed to create the storage directory");
    std::fs::write(
        &path,
        "Hello World!",
    )
    .expect("Failed to write to file");
    tracing::debug!("Written to file: {}", &path.to_string_lossy());
    let app = storage::app(shared);
    let req = axum::http::Request::builder()
        .method("GET")
        .uri(format!("/download/{}", file_id))
        .header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", &token),
        )
        .body(axum::body::Body::empty())
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    use http_body_util::BodyExt;
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(bytes, "Hello World!");
}
