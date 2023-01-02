use crate::{config, handler};
use actix_web::{web, App, HttpServer};

pub async fn start() -> std::io::Result<()> {
    let address = &config().webserver.bind_address;
    HttpServer::new(move || {
        App::new()
            .route(
                "/hash/{resource}/{file}",
                web::get().to(handler::get_hash::get_hash_endpoint),
            )
            .route(
                "/file/{resource}/{file}",
                web::get().to(handler::get_file::get_file_endpoint),
            )
    })
    .bind(address)?
    .run()
    .await
}
