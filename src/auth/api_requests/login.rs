use crate::auth::cookies::headers::{
    get_new_access_token_cookie_header, get_new_refresh_token_cookie_header,
    get_new_session_uuid_cookie_header,
};
use crate::auth::utils::password::verify_password;
use actix_web::http::header;
use actix_web::{web, HttpResponse};

use crate::auth::user::{User, UserInfo};

pub async fn proceed_with_login(user_data: &User, stored_user: &UserInfo) -> HttpResponse {
    if let Ok(valid) = verify_password(&user_data.password, &stored_user.password).await {
        if valid {
            let pool = crate::db::create_pool().await.unwrap();
            let session_uuid = crate::db::create_session(stored_user, &pool).await.unwrap();

            let access_token_header = get_new_access_token_cookie_header(stored_user).await;
            let refresh_token_header = get_new_refresh_token_cookie_header(stored_user).await;
            let session_header = get_new_session_uuid_cookie_header(&session_uuid).await;

            return HttpResponse::Ok()
                .append_header((header::SET_COOKIE, access_token_header))
                .append_header((header::SET_COOKIE, refresh_token_header))
                .append_header((header::SET_COOKIE, session_header))
                .finish();
        }
    }
    HttpResponse::Unauthorized().body("Invalid username or password")
}

pub async fn login(user_data: web::Json<User>) -> impl actix_web::Responder {
    let pool = crate::db::create_pool().await.unwrap();

    match crate::db::get_user_from_db(&user_data.username, &pool).await {
        Ok(stored_user) => proceed_with_login(&user_data, &stored_user).await,
        Err(_) => HttpResponse::Unauthorized().body("Invalid username or password"),
    }
}
