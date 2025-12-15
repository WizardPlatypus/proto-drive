use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use password_hash::rand_core::OsRng;
use std::env;

fn main() {
    let mut args = env::args().skip(1);

    let login = args.next().unwrap_or_else(|| {
        eprintln!("usage: user_sql <login> <password>");
        std::process::exit(1);
    });

    let password = args.next().unwrap_or_else(|| {
        eprintln!("usage: user_sql <login> <password>");
        std::process::exit(1);
    });

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("password hashing failed")
        .to_string();

    let id = uuid::Uuid::new_v4();

    // You may or may not want to print login; usually not needed
    println!("INSERT INTO users (id, login, phc)");
    println!("VALUES ('{}'::UUID, '{}', '{}');", &id, &login, &hash);
    println!("INSERT INTO configs (user_id)");
    println!("VALUES ('{}'::UUID);", &id);
}
