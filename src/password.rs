use argon2::Argon2;
use password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

pub fn hash_password(password: &str) -> password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> password_hash::Result<()> {
    let hash = PasswordHash::new(password_hash)?;
    Argon2::default().verify_password(password.as_bytes(), &hash)
}
