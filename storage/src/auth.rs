use argon2::{Argon2, PasswordHash, PasswordVerifier};
use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};

pub fn hash_password(password: &str) -> password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon.hash_password(password.as_bytes(), &salt)?;

    Ok(hash.to_string())
}

pub fn verify_password(password: &str, phc: &str) -> password_hash::Result<bool> {
    let hash = PasswordHash::new(phc)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &hash).is_ok())
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