use actix_web::{error, web, App, HttpServer, Result};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "invalid name: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError { name: "longinuses" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}
