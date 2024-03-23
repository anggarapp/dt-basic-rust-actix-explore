use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix")
}
#[get("/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

async fn manual_greet() -> impl Responder {
    HttpResponse::Ok().body("Hey Kiddos")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
            .route("/eiy", web::post().to(manual_greet))
    })
    .bind(("127.0.0.1", 8099))?
    .run()
    .await
}
