use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::auth::token::constants::{REFRESH_ALGORITHM, REFRESH_TOKEN_EXPIRATION};
use crate::auth::user::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub session: String,
    pub sub: String,
    pub exp: i64,
}

pub async fn create_refresh_token(user: &UserInfo) -> Result<String, Box<dyn std::error::Error>> {
    let pool = crate::db::create_pool().await?;
    let secret_refresh_key = crate::db::get_secret_refresh_key(user.user_id, &pool).await?;
    let session_uuid = crate::db::get_session_uuid(user.user_id, &pool).await?;

    let refresh_claims = RefreshClaims {
        session: session_uuid,
        sub: user.username.clone(),
        exp: (Utc::now().timestamp() + REFRESH_TOKEN_EXPIRATION),
    };

    let refresh_token = encode(
        &Header::new(REFRESH_ALGORITHM),
        &refresh_claims,
        &EncodingKey::from_secret(secret_refresh_key.as_ref()),
    )?;
    Ok(refresh_token)
}

pub async fn validate_refresh_token(
    token: &str,
    user_id: i64,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let user = crate::db::get_user_from_db_with_user_id(user_id, pool).await?;
    let secret_key = crate::db::get_secret_refresh_key(user.user_id, pool).await?;

    let refresh_claims = decode::<RefreshClaims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(REFRESH_ALGORITHM),
    )?;

    let current_timestamp = Utc::now().timestamp();
    Ok(refresh_claims.claims.exp - current_timestamp > 0)
}
