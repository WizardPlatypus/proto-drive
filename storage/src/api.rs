use std::io::Write;
use std::path::{Component, PathBuf};
use std::sync::Arc;

use axum::body::Body;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use axum::{
    Json,
    extract::{FromRequestParts, Multipart, State},
    http::{StatusCode, request::Parts},
};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;

use crate::{auth, db};

#[derive(Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

pub type LoginRequest = Credentials;
pub type RegisterRequest = Credentials;

#[derive(Serialize)]
pub struct AuthOk {
    pub access_token: String,
}

// POST /auth/register
pub async fn register(
    State(shared): State<Shared>,
    Json(req): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    let _user_id = auth::register_user(&shared.pool, &req.login, &req.password)
        .await
        .map_err(map_auth_error)?;
    Ok(StatusCode::CREATED)
}

pub fn map_auth_error(e: auth::Error) -> StatusCode {
    use auth::Error::*;
    match e {
        InvalidCredentials => StatusCode::UNAUTHORIZED,
        PasswordHash(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// POST /auth/login
pub async fn login(
    State(shared): State<Shared>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthOk>, StatusCode> {
    let user_id = auth::login_user(&shared.pool, &req.login, &req.password)
        .await
        .map_err(map_auth_error)?;
    let token = auth::jwt::issue(user_id, &shared.jwt_secret, Duration::minutes(30))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(AuthOk {
        access_token: token,
    }))
}

#[derive(Clone)]
pub struct Shared {
    pub pool: PgPool,
    pub jwt_secret: Arc<[u8]>,
    pub root: PathBuf,
}

impl Shared {
    pub async fn from_env() -> Result<Shared, Error> {
        let db_connection_string = std::env::var("DATABASE_URL")?;
        let jwt_secret = Arc::from(std::env::var("JWT_SECRET")?.as_bytes());
        let root = std::path::PathBuf::from(&std::env::var("ROOT")?);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&db_connection_string)
            .await?;
        Ok(Shared {
            pool,
            jwt_secret,
            root,
        })
    }
}

impl FromRequestParts<Shared> for auth::User {
    type Rejection = StatusCode;

    fn from_request_parts(
        parts: &mut Parts,
        state: &Shared,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            let jwt_secret = &state.jwt_secret;
            let auth_header = parts
                .headers
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .ok_or(StatusCode::UNAUTHORIZED)?;
            let claims = auth::jwt::validate(auth_header, jwt_secret)
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            Ok(Self { id: claims.sub })
        })
    }
}

fn sanitize_destination(input: &str) -> String {
    if !input.starts_with("/") {
        tracing::warn!("The path was not absolute");
    }
    let path = std::path::Path::new(input);
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::RootDir => {}
            Component::Normal(os) => {
                let part = os.to_string_lossy();
                parts.push(sanitize_filename::sanitize(part));
            }
            _ => {
                tracing::warn!("Invalid path component")
            }
        }
    }
    if parts.is_empty() {
        String::from("/")
    } else {
        format!("/{}/", parts.join("/"))
    }
}

// POST /upload
pub async fn upload_file(
    State(shared): State<Shared>,
    user: auth::User,
    mut multipart: Multipart,
) -> Result<StatusCode, Error> {
    let mut destination = None;
    let mut file = None;
    while let Some(mut field) = multipart.next_field().await? {
        match field.name() {
            Some("file") if field.file_name().is_some() => {
                tracing::info!("Matched a \"file\" field");
                let name = sanitize_filename::sanitize(field.file_name().unwrap());
                if name.is_empty() {
                    tracing::error!(name: "filename_empty", "File name contained no characters once sanitized");
                    return Err(Error::BadRequest(String::from(
                        "File name contained no characters once sanitized",
                    )));
                }
                tracing::debug!("Sanitized name: \"{}\"", &name);
                let file_id = db::file::create(&shared.pool, &name, None, &user.id).await?;
                tracing::trace!("Acquired file UUID");
                let temp_path = std::path::PathBuf::new()
                    .join("temp")
                    .join(user.id.to_string())
                    .join(file_id.to_string());
                tracing::debug!("Temp path: \"{}\"", &temp_path.to_string_lossy());
                db::file::r#move(&shared.pool, &file_id, &temp_path.to_string_lossy()).await?;
                tracing::trace!("Updated DB path");
                std::fs::create_dir_all(shared.root.join(&temp_path).parent().unwrap())?;
                tracing::trace!("Created intermediate directores");
                let mut temp = std::fs::File::create(shared.root.join(&temp_path))?;
                tracing::trace!("Opened the temp file");
                while let Some(chunk) = field.chunk().await? {
                    temp.write_all(&chunk)?;
                }
                temp.sync_all()?;
                tracing::trace!("Processed all chunks");
                file = Some((name, file_id, temp_path));
            }
            Some("destination") => {
                tracing::trace!("Matched a \"destination\" field");
                destination = Some(sanitize_destination(&field.text().await?));
            }
            _ => {
                tracing::trace!("Skipped a field");
            }
        }
    }
    let destination = destination.ok_or(Error::BadRequest(String::from(
        "Multipart missing \"destination\" field.",
    )))?;
    let (name, file_id, temp_path) = file.ok_or(Error::BadRequest(String::from(
        "Multipart missing \"file\" field.",
    )))?;
    let new_name = destination + &name;
    tracing::debug!("New name: {}", &new_name);
    db::file::rename(&shared.pool, &file_id, &new_name).await?;
    tracing::info!("Renamed DB file");
    let new_path = std::path::PathBuf::new()
        .join("storage")
        .join(user.id.to_string())
        .join(file_id.to_string());
    tracing::debug!("New path: {}", &new_path.to_string_lossy());
    db::file::r#move(
        &shared.pool,
        &file_id,
        &shared.root.join(&new_path).to_string_lossy(),
    )
    .await?;
    tracing::info!("Moved DB file");
    std::fs::create_dir_all(shared.root.join(&new_path).parent().unwrap())?;
    tracing::trace!("Created intermediate directores");
    tokio::fs::rename(shared.root.join(temp_path), shared.root.join(new_path)).await?;
    tracing::info!("Moved physical file");
    Ok(StatusCode::CREATED)
}

// GET /download/{file_id}
pub async fn download_file(
    State(shared): State<Shared>,
    user: auth::User,
    axum::extract::Path(file_id): axum::extract::Path<uuid::Uuid>,
) -> Result<impl IntoResponse, Error> {
    let file = db::file::find_by_id(&shared.pool, &file_id)
        .await?
        .ok_or(Error::NotFound(String::from(
            "A file with that UUID does not exist",
        )))?;
    tracing::trace!("Found the file");
    if file.owned_by != user.id {
        return Err(Error::Forbidden(String::from(
            "You do not have access to that file",
        )));
    }
    if file.path.is_none() {
        return Err(Error::BadRequest(String::from("Cannot download a folder")));
    }
    tracing::trace!("File is not a folder");
    let path = shared.root.join(file.path.unwrap());
    tracing::debug!("Looking for file at {}", path.to_string_lossy());
    let file = tokio::fs::File::open(&path).await?;
    tracing::trace!("File opened");
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);
    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub regex: String,
    pub order_by: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// GET /files/{query}
pub async fn find_files(
    State(shared): State<Shared>,
    user: auth::User,
    Query(SearchQuery {
        regex,
        order_by,
        limit,
        offset,
    }): Query<SearchQuery>,
) -> Result<Json<Vec<db::File>>, Error> {
    if regex.trim().is_empty() {
        return Err(Error::BadRequest(String::from(
            "Search term may not be empty",
        )));
    }
    let files = db::file::regex(
        &shared.pool,
        &regex,
        &user.id,
        &order_by.unwrap_or(String::from("name")),
        limit.unwrap_or(100),
        offset.unwrap_or(0),
    )
    .await?;
    Ok(Json(files))
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Environment error")]
    Environment(#[from] std::env::VarError),
    #[error("SQLx error")]
    Database(#[from] sqlx::error::Error),
    #[error("Multipart protocol error")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
    #[error("IO error")]
    InputOutput(#[from] std::io::Error),
    #[error("BAD_REQUEST generic error")]
    BadRequest(String),
    #[error("NOT_FOUND generic error")]
    NotFound(String),
    #[error("FORBIDDEN generic error")]
    Forbidden(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Environment(e) => {
                tracing::error!(name: "environment_error", "{}", e.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Something went wrong."),
                )
            }
            Error::Database(e) => {
                tracing::error!(name: "database_error", "{}", e.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Something went wrong."),
                )
            }
            Error::InputOutput(e) => {
                tracing::error!(name: "io_error", "{}", e.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Something went wrong."),
                )
            }
            Error::Multipart(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Error::Forbidden(message) => (StatusCode::FORBIDDEN, message),
        };

        // Create the JSON response body
        let body = Json(ErrorResponse { message });

        // Build and return the final Axum Response
        (status, body).into_response()
    }
}
