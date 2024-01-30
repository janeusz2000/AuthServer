use crate::auth::token::access_token::create_access_token;
use crate::auth::token::refresh_token::create_refresh_token;
use actix_web::http::header::HeaderValue;
use cookie::{Cookie, SameSite};

use crate::auth::user::UserInfo;

// Created cookies needs to have two methods set up:
// a) set_http_only(true) - This prevents access via client-side scripts
// b) set_secure(true)    - Ensures cookie is only transmitted over HTTPS

pub async fn get_new_access_token_cookie_header(stored_user: &UserInfo) -> HeaderValue {
    // TODO: make safe version
    let token = create_access_token(stored_user).await.unwrap();
    let mut access_token_cookie = Cookie::new("token", token);
    access_token_cookie.set_http_only(true);
    access_token_cookie.set_secure(true);
    access_token_cookie.set_same_site(SameSite::Strict);
    HeaderValue::from_str(&access_token_cookie.to_string()).unwrap()
}

pub async fn get_new_refresh_token_cookie_header(stored_user: &UserInfo) -> HeaderValue {
    // TODO: make safe version
    let refresh_token = create_refresh_token(stored_user).await.unwrap();
    let mut refresh_token_cookie = Cookie::new("refresh_token", refresh_token);
    refresh_token_cookie.set_http_only(true);
    refresh_token_cookie.set_secure(true);
    refresh_token_cookie.set_same_site(SameSite::Strict);
    HeaderValue::from_str(&refresh_token_cookie.to_string()).unwrap()
}

pub async fn get_new_session_uuid_cookie_header(session_uuid: &String) -> HeaderValue {
    let mut session_cookie = Cookie::new("session", session_uuid);
    session_cookie.set_http_only(true);
    session_cookie.set_secure(true);
    session_cookie.set_same_site(SameSite::Strict);
    HeaderValue::from_str(&session_cookie.to_string()).unwrap()
}

pub fn extract_refresh_token(
    req: &actix_web::HttpRequest,
) -> Result<String, actix_web::error::Error> {
    match req.cookie("refresh_token") {
        Some(cookie) => Ok(cookie.value().to_string()),
        None => Err(actix_web::error::ErrorBadRequest(
            "No refresh token in cookie",
        )),
    }
}
