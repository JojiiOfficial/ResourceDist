use crate::config;
use actix_web::HttpResponse;

pub async fn healthy_endpoint() -> HttpResponse {
    let healthy = crate::check_dirs(config()).is_ok();
    if healthy {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
