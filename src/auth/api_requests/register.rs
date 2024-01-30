use actix_web::{web, HttpResponse};
use serde::Serialize;

use crate::auth::token::constants::{REFRESH_KEY_LENGTH, SECRET_KEY_LENGTH};
use crate::auth::user::RegisterUserInfo;
use crate::auth::utils::errors::HashPasswordError;
use crate::auth::utils::password::hash_password;
use crate::logging::log::log_error;
use crate::utils::random::random_string;

#[derive(Serialize)]
struct ConflictResponse {
    message: String,
}

pub async fn create_user(
    username: &str,
    password: &str,
    email: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = crate::db::create_pool().await.unwrap();
    match hash_password(password) {
        Ok(hashed_password) => {
            crate::db::store_new_user(username, &hashed_password, email, &pool).await?;
            let user = crate::db::get_user_from_db(username, &pool).await.unwrap();

            let secret_access_key = random_string(SECRET_KEY_LENGTH);
            crate::db::store_secret_access_key(user.user_id, &secret_access_key, &pool).await?;

            let refresh_key = random_string(REFRESH_KEY_LENGTH);
            crate::db::store_secret_refresh_key(user.user_id, &refresh_key, &pool).await?;
            Ok(())
        }
        Err(e) => {
            log_error(&e.to_string());
            Err(Box::new(HashPasswordError))
        }
    }
}

pub async fn register(user_data: web::Json<RegisterUserInfo>) -> impl actix_web::Responder {
    let pool = crate::db::create_pool().await.unwrap();

    match crate::db::does_username_exists(&pool, &user_data.username).await {
        Ok(does_user_exists) => {
            if does_user_exists {
                let response = ConflictResponse {
                    message: "Username already exists.".into(),
                };
                return HttpResponse::Conflict().json(response);
            }

            match create_user(&user_data.username, &user_data.password, &user_data.email).await {
                Ok(_) => HttpResponse::Ok().into(),
                Err(_) => HttpResponse::InternalServerError().into(),
            }
        }
        Err(e) => {
            log_error(&e.to_string());
            HttpResponse::InternalServerError().into()
        }
    }
}
