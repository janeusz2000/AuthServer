use crate::auth::cookies::errors::NoValueInCookie;
use crate::db::get_user_id_with_session_uuid;

pub fn get_token_from_cookie(
    req: &actix_web::HttpRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    match req.cookie("token") {
        Some(cookie) => Ok(cookie.value().to_string()),
        None => Err(Box::new(NoValueInCookie)),
    }
}

pub fn get_session_uuid_from_cookie(
    req: &actix_web::HttpRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    match req.cookie("session") {
        Some(cookie) => Ok(cookie.value().to_string()),
        None => Err(Box::new(NoValueInCookie)),
    }
}

pub async fn extract_user_id_from_cookie(
    req: &actix_web::HttpRequest,
) -> Result<i64, Box<dyn std::error::Error>> {
    match get_session_uuid_from_cookie(req) {
        Ok(session_uuid) => get_user_id_with_session_uuid(&session_uuid).await,
        Err(e) => Err(e),
    }
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
