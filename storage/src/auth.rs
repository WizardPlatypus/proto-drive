use argon2::{Argon2, PasswordHash, PasswordVerifier};
use password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

pub mod jwt;

pub fn hash_password(password: &str) -> password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon.hash_password(password.as_bytes(), &salt)?;

    Ok(hash.to_string())
}

pub fn verify_password(password: &str, phc: &str) -> password_hash::Result<bool> {
    let hash = PasswordHash::new(phc)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Password hashing error")]
    PasswordHash(password_hash::Error),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

pub async fn register_user(pool: &PgPool, login: &str, password: &str) -> Result<Uuid, Error> {
    let phc = hash_password(password).map_err(Error::PasswordHash)?;
    let mut tx = pool.begin().await?;
    let user_id = crate::db::user::create(&mut *tx, login, &phc).await?;
    crate::db::config::init(&mut *tx, &user_id).await?;
    tx.commit().await?;
    Ok(user_id)
}

pub async fn login_user(pool: &PgPool, login: &str, password: &str) -> Result<Uuid, Error> {
    let user = crate::db::user::find_by_login(pool, login)
        .await?
        .ok_or(Error::InvalidCredentials)?;
    let ok = verify_password(password, &user.phc).map_err(Error::PasswordHash)?;
    if ok {
        Ok(user.id)
    } else {
        Err(Error::InvalidCredentials)
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
}

#[cfg(test)]
mod tests {
    #[test]
    fn password_roundtrip() {
        let password = "avada kedavra";
        let phc = super::hash_password(password).unwrap();

        assert!(super::verify_password(password, &phc).unwrap());
        assert!(!super::verify_password("wrong", &phc).unwrap());
    }
}
