use argon2::{Argon2, PasswordHash, PasswordVerifier};
use password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api;

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

pub async fn register_user(pool: &PgPool, login: &str, password: &str) -> Result<Uuid, api::Error> {
    let phc = hash_password(password)?;
    let mut tx = pool.begin().await?;
    let user_id = crate::db::user::create(&mut *tx, login, &phc).await?;
    crate::db::config::init(&mut *tx, &user_id).await?;
    tx.commit().await?;
    Ok(user_id)
}

pub async fn login_user(pool: &PgPool, login: &str, password: &str) -> Result<Uuid, api::Error> {
    let user = crate::db::user::find_by_login(pool, login)
        .await?
        .ok_or(api::Error::Unauthorized(String::from("Invalid login")))?;
    let ok = verify_password(password, &user.phc)?;
    if ok {
        Ok(user.id)
    } else {
        Err(api::Error::Unauthorized(String::from(
            "Invalid credentials",
        )))
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
