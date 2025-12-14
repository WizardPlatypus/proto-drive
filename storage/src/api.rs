use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth;

#[derive(Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

pub type LoginRequest = Credentials;
pub type RegisterRequest = Credentials;

#[derive(Serialize)]
pub struct AuthOk {
    pub user_id: uuid::Uuid,
}

// POST /auth/register
pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthOk>, StatusCode> {
    let user_id = auth::register_user(&pool, &req.login, &req.password)
        .await
        .map_err(map_auth_error)?;
    Ok(Json(AuthOk { user_id }))
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
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthOk>, StatusCode> {
    let user_id = auth::login_user(&pool, &req.login, &req.password)
        .await
        .map_err(map_auth_error)?;
    Ok(Json(AuthOk { user_id }))
}
