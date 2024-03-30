use crate::routes::api;
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub fn start(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(api))
            .route("/", web::post().to(api))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
