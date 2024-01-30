pub async fn ping(_req: actix_web::HttpRequest) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Pong!")
}
