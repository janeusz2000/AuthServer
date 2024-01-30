use crate::utils::random::random_string;
use argon2::Config;

const SALT_LENGTH: usize = 16;

pub async fn verify_password(
    password: &str,
    stored_password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(argon2::verify_encoded(
        stored_password,
        password.as_bytes(),
    )?)
}

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = random_string(SALT_LENGTH);
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
}
