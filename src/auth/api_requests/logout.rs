use actix_web::HttpResponse;

use crate::auth::cookies::utils::get_session_uuid_from_cookie;
use crate::auth::utils::validate_request::validate_http_request;
use crate::db::drop_session;
use crate::logging::log::log_warn;

pub async fn logout(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    match validate_http_request(&req).await {
        Ok(_) => {
            let session_uuid = get_session_uuid_from_cookie(&req).unwrap();
            if drop_session(&session_uuid).await {
                return HttpResponse::Ok().body("logout sucessfull");
            }
            HttpResponse::InternalServerError().into()
        }
        Err(e) => {
            log_warn(&e.to_string());
            HttpResponse::Unauthorized().body("You are not authorized to make this request")
        }
    }
}
