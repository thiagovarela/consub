use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

/// TODO: Make this run in a tokio thread/async
pub fn hash_password(raw: String) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // TODO: Return here to choose parameters
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(raw.as_bytes(), &salt)
        .map_err(|_| anyhow!("Unable to hash password"))?
        .to_string();

    Ok(password_hash)
}

/// TODO: Make this run in a tokio thread/async
pub fn verify_password(raw: String, hash: String) -> Result<(), anyhow::Error> {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(&hash).map_err(|_| anyhow!("Unable to hash password"))?;

    match argon2
        .verify_password(raw.as_bytes(), &password_hash)
        .map_err(|_| anyhow!("Verify password failed"))
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
