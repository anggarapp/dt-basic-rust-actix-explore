// start worker
// use actix_web::{web, App, HttpResponse, HttpServer};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().route(
//             "/",
//             web::get().to(|| async { HttpResponse::Ok().body("yay") }),
//         )
//     })
//     .workers(2) //set count of workers manually
//     // .bind(("127.0.0.1", 8099)) Alternative Bind
//     .bind("127.0.0.1:8099")?
//     .run()
//     .await
// }
// end worker

// start

// generate ssl key with this command:
// openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem  -days 365 -sha256 -subj "/C=ID/ST=DKI/L=Jakarta/O=RustLang/OU=Org/CN=localhost"

use actix_web::{get, App, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index() -> impl Responder {
    "Welcome with SSL"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    ssl_builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:8099", ssl_builder)?
        .run()
        .await
}

// end
