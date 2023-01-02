use actix_web::{App, HttpServer};

use crate::config;

pub async fn start() -> std::io::Result<()> {
    let address = &config().webserver.bind_address;
    HttpServer::new(move || {
        App::new()
        //.route("/login", web::post().to(user::login))
    })
    .bind(address)?
    .run()
    .await
}
