// start init url dispatcher
// use actix_web::{guard, web, App, HttpResponse, HttpServer};
// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hey Kiddos!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(web::resource("/prefix").to(index))
//             .service(
//                 web::resource("user/{username}")
//                     .name("user_detail")
//                     .guard(guard::Header("content-type", "application/json"))
//                     .route(web::get().to(HttpResponse::OK))
//                     .route(web::put().to(HttpResponse::Ok)),
//             )
//             .service(
//                 web::resource("/path").route(
//                     web::route()
//                         .guard(guard::Get())
//                         .guard(guard::Header("content-type", "text/plain"))
//                         .to(HttpResponse::Ok),
//                 ),
//             )
//     })
//     .bind(("127.0.0.1", 8099))?
//     .run()
//     .await
// }
// end init url dispatcher

// start scope as prefix
// use actix_web::{get, web, App, HttpResponse, HttpServer};

// #[get("/show")]
// async fn show_users() -> HttpResponse {
//     HttpResponse::Ok().body("Show Users!")
// }

// #[get("/show/{id}")]
// async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             web::scope("/users")
//                 .service(show_users)
//                 .service(user_detail),
//         )
//     })
//     .bind(("127.0.0.1", 8099))?
//     .run()
//     .await
// }
// end scope as prefix

// start scope as prefix and route guard
// use actix_web::{guard, web, App, HttpResponse, HttpServer};

// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello Kiddos!")
// }

// async fn show_users() -> HttpResponse {
//     HttpResponse::Ok().body("Show Users!")
// }

// async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             web::scope("/api")
//                 .service(web::resource("/hello").route(web::route().guard(guard::Get()).to(index)))
//                 .service(web::resource("/users").to(show_users))
//                 .service(web::resource("/users/{id}").to(user_detail)),
//         )
//     })
//     .bind(("127.0.0.1", 8099))?
//     .run()
//     .await
// }
// end scope as prefix and route guard

// start match_info
use actix_web::{get, App, HttpRequest, HttpServer, Result};

#[get("/version/{v1}/{v2}")]
async fn index(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();

    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();

    Ok(format!(
        "Value v1: {}, v2: {}, v3: {}, v4: {}",
        v1, v2, v3, v4
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}
// end match_info
