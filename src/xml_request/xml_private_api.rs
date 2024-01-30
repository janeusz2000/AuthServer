use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct XMLContent {
    content: String,
}

// TODO: [SPIR-100] This must be done with token authorization
pub async fn send_xml(content_data: actix_web::web::Json<XMLContent>) -> impl actix_web::Responder {
    match crate::xml_request::xml_post::send_xml_request(&content_data.content).await {
        Ok(_) => actix_web::HttpResponse::Ok(),
        Err(e) => {
            crate::logging::log::log_error(&format!(
                "There was en error with sending xml data to server {}",
                e
            ));
            actix_web::HttpResponse::InternalServerError()
        }
    }
}
