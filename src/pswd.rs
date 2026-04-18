use crate::prelude::*;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use tokio::time::Instant;

pub fn password_hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())?)
}

pub async fn password_verify(password: &str, hash: &str) -> Result<bool> {
    let fixed_time_end = Instant::now() + crate::constants::FIXED_TIMESTAMP_DELAY;

    match password_verify_variable_time(password, hash) {
        Ok(true) => Ok(true),
        other => {
            tokio::time::sleep_until(fixed_time_end).await;
            other
        }
    }
}

fn password_verify_variable_time(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
