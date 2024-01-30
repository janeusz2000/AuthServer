use crate::auth::cookies::utils::{extract_user_id_from_cookie, get_token_from_cookie};
use crate::auth::token::access_token::{validate_token, Claims};
use crate::auth::token::errors::TokenValidationNotSuccessfull;
use crate::logging::log::log_warn;

async fn proceed_with_validation(
    token: &str,
    req: &actix_web::HttpRequest,
) -> Result<Claims, Box<dyn std::error::Error>> {
    let user_id = extract_user_id_from_cookie(req).await?;
    match validate_token(token, user_id).await {
        Ok(claims) => Ok(claims),
        Err(e) => {
            log_warn(&e.to_string());
            Err(Box::new(TokenValidationNotSuccessfull))
        }
    }
}

pub async fn validate_http_request(
    req: &actix_web::HttpRequest,
) -> Result<Claims, Box<dyn std::error::Error>> {
    match get_token_from_cookie(req) {
        Ok(token) => proceed_with_validation(&token, req).await,
        Err(e) => Err(e),
    }
}
