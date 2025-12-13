use serde::{Deserialize, Serialize};

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
