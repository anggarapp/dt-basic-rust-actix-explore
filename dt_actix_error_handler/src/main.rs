// start error handler use default impl
// use actix_web::{error, web, App, HttpServer, Result};
// use derive_more::{Display, Error};

// #[derive(Debug, Display, Error)]
// #[display(fmt = "invalid name: {}", name)]
// struct MyError {
//     name: &'static str,
// }

// impl error::ResponseError for MyError {}

// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError { name: "longinuses" })
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(web::resource("/").to(index)))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }
// end error handler use default impl

// start error handler use default impl

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    web, App, HttpResponse, HttpServer, Result,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "Internal error")]
    InternalError,

    #[display(fmt = "Bad Request")]
    BadClientData,

    #[display(fmt = "Timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError::InternalError)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}

// end error handler use default impl
