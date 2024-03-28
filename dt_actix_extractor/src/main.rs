use actix_web::{
    get,
    web::{self, Json},
    App, HttpRequest, HttpServer, Responder, Result,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: i32,
    username: String,
}

async fn index(
    json: web::Json<MyInfo>,
    patt: web::Path<(String, String)>,
    evt: web::Json<Event>,
) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(factory)
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}
