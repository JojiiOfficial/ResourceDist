use actix_web::HttpResponse;

pub async fn ready_endpoint() -> HttpResponse {
    HttpResponse::Ok().finish()
}
