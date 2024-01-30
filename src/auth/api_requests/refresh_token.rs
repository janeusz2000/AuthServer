use actix_web::http::header;
use actix_web::HttpResponse;

use crate::auth::cookies::headers::{
    get_new_access_token_cookie_header, get_new_refresh_token_cookie_header,
};
use crate::auth::cookies::utils::{extract_refresh_token, extract_user_id_from_cookie};
use crate::auth::token::refresh_token::validate_refresh_token;

pub async fn refresh_token(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    match (
        extract_refresh_token(&req),
        extract_user_id_from_cookie(&req).await,
    ) {
        (Ok(token), Ok(user_id)) => {
            let pool = crate::db::create_pool().await.unwrap();

            if crate::db::does_user_id_exists(&pool, user_id)
                .await
                .unwrap()
                && validate_refresh_token(&token, user_id, &pool)
                    .await
                    .unwrap()
            {
                let pool = crate::db::create_pool().await.unwrap();
                let user = crate::db::get_user_from_db_with_user_id(user_id, &pool)
                    .await
                    .unwrap();
                let token_cookie_header = get_new_access_token_cookie_header(&user).await;
                let refresh_cookie_header = get_new_refresh_token_cookie_header(&user).await;
                return HttpResponse::Ok()
                    .append_header((header::SET_COOKIE, token_cookie_header))
                    .append_header((header::SET_COOKIE, refresh_cookie_header))
                    .finish();
            }

            HttpResponse::Unauthorized().into()
        }
        _ => HttpResponse::BadRequest().into(),
    }
}
