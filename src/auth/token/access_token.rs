use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::auth::token::constants::{ACCESS_TOKEN_EXPIRATION, ALGORITHM};
use crate::auth::token::errors::ExpiredAccessTokenError;
use crate::auth::user::UserInfo;
use crate::logging::log::log_info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp: i64,
}

impl std::fmt::Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let naive_datetime = NaiveDateTime::from_timestamp_opt(self.exp, 0);
        let exp_date = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime.unwrap(), Utc);
        write!(
            f,
            "Claims {{ username: {}, exp: {} }}",
            self.username,
            exp_date.format("%H:%M:%S %d-%m-%Y")
        )
    }
}

pub async fn create_access_token(user: &UserInfo) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims {
        username: user.username.clone(),
        exp: (Utc::now().timestamp() + ACCESS_TOKEN_EXPIRATION),
    };

    let pool = crate::db::create_pool().await?;

    let secret_key = crate::db::get_secret_access_key(user.user_id, &pool).await?;
    log_info(&format!("secret key : {}", &secret_key));
    let token = encode(
        &Header::new(ALGORITHM),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap();
    Ok(token)
}

pub async fn validate_token(
    token: &str,
    user_id: i64,
) -> Result<Claims, Box<dyn std::error::Error>> {
    let pool = crate::db::create_pool().await?;
    let secret_key = crate::db::get_secret_access_key(user_id, &pool).await?;
    log_info(&format!(
        "obtaining secret_key from database: {}",
        &secret_key
    ));
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(ALGORITHM),
    )?;

    log_info(&format!("obtained claims:\t{}", claims.claims));
    let current_time = Utc::now().timestamp();
    if current_time - claims.claims.exp > 0 {
        return Err(Box::new(ExpiredAccessTokenError));
    }
    Ok(claims.claims)
}
