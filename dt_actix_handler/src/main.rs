// start custom response
// use actix_web::{
//     body::BoxBody, get, http::header::ContentType, App, HttpRequest, HttpResponse, HttpServer,
//     Responder,
// };
// use serde::Serialize;

// #[derive(Serialize)]
// struct MyCustomReturn {
//     name: &'static str,
// }

// impl Responder for MyCustomReturn {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap();
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }

// #[get("/")]
// async fn index() -> impl Responder {
//     MyCustomReturn { name: "Corpencius" }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }
// end custom Response

use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"arise")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(stream))
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}
