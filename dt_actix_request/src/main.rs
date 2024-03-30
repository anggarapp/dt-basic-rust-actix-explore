// start manual handle request payload
// use actix_web::{error, post, web, App, Error, HttpResponse, HttpServer, Result};
// use futures::StreamExt;
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// struct Person {
//     name: String,
//     age: u32,
// }

// const MAX_SIZE: usize = 256_144; // 256k

// #[post("/")]
// async fn index(mut payload: web::Payload) -> Result<HttpResponse, Error> {
//     let mut body = web::BytesMut::new();

//     while let Some(chunk) = payload.next().await {
//         let chunk = chunk?;

//         if (body.len() + chunk.len()) > MAX_SIZE {
//             return Err(error::ErrorBadRequest("overflow"));
//         }

//         body.extend_from_slice(&chunk);
//     }

//     let obj = serde_json::from_slice::<Person>(&body)?;

//     Ok(HttpResponse::Ok().json(obj)) // send response
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }
// end manual handle request payload

// start manual handle basic content encoding

// use actix_web::{web, App, HttpResponse, HttpServer, Result};
// use flate2::read::GzDecoder;
// use futures::StreamExt;
// use std::io::Read;

// async fn index(mut body: web::Payload) -> Result<HttpResponse> {
//     let mut bytes = web::BytesMut::new();

//     while let Some(item) = body.next().await {
//         bytes.extend_from_slice(&item?);
//     }

//     let mut decoder = GzDecoder::new(&bytes[..]);
//     let mut decoded = String::new();

//     decoder.read_to_string(&mut decoded)?;

//     Ok(HttpResponse::Ok().body(decoded))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(web::resource("/").route(web::post().to(index))))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }

// end manual handle basic content encoding

// start showing byte length

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use futures::StreamExt;

#[get("/")]
async fn index(mut body: web::Payload) -> Result<HttpResponse> {
    let mut bytes = web::BytesMut::new();

    while let Some(item) = body.next().await {
        let item = item?;

        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    let num_bytes = bytes.len();
    let response_body = format!("Received {} bytes", num_bytes);

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}

// end showing byte length
