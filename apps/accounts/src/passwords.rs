use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use crate::error::Error;

pub fn hash_password(raw: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(raw.as_bytes(), &salt)
        .map_err(|_| Error::PasswordError("Unable to hash password".into()))?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(raw: String, hash: String) -> Result<(), Error> {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(&hash)
        .map_err(|_| Error::PasswordError("Unable to hash password".into()))?;

    match argon2
        .verify_password(raw.as_bytes(), &password_hash)
        .map_err(|_| Error::PasswordError("Verify password failed".into()))
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
