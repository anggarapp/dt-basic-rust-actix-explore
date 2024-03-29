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

// // start error handler use custom impl

// use actix_web::{
//     error,
//     http::{header::ContentType, StatusCode},
//     web, App, HttpResponse, HttpServer, Result,
// };
// use derive_more::{Display, Error};

// #[derive(Debug, Display, Error)]
// enum MyError {
//     #[display(fmt = "Internal error")]
//     InternalError,

//     #[display(fmt = "Bad Request")]
//     BadClientData,

//     #[display(fmt = "Timeout")]
//     Timeout,
// }

// impl error::ResponseError for MyError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::json())
//             .body(self.to_string())
//     }

//     fn status_code(&self) -> StatusCode {
//         match *self {
//             MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//             MyError::BadClientData => StatusCode::BAD_REQUEST,
//             MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
//         }
//     }
// }

// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError::InternalError)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(web::resource("/").to(index)))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }

// // end error handler use custom impl

// start error handler using helper function

// use actix_web::{error, web, App, HttpServer, Result};
// struct MyError {
//     name: &'static str,
// }

// async fn index() -> Result<&'static str> {
//     let result: Result<&'static str, MyError> = Err(MyError { name: "Longinus" });
//     Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(web::resource("/").to(index)))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }

// end error handler using helper function

// start error handler with split logic with log
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    post, App, HttpResponse, HttpServer, Result,
};
use derive_more::{Display, Error};
use env_logger;
use log::error;
use serde;

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Internal error")]
    InternalError,

    #[display(fmt = "Invalid Input")]
    InvalidInput,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();
        error!("{} - Status code: - {}", error_message, status_code);
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::InvalidInput => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct MyData {
    value: String,
}

fn just_fails() -> Result<(), UserError> {
    Err(UserError::InternalError)
}

#[post("/proc")]
async fn proc(data: actix_web::web::Json<MyData>) -> Result<&'static str, UserError> {
    if data.value.is_empty() {
        return Err(UserError::InvalidInput);
    }

    just_fails().map_err(|_e| UserError::InternalError)?;
    Ok("Success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| App::new().service(proc))
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}
// end error handler with split logic with log
