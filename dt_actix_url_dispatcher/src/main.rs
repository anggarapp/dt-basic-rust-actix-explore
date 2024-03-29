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
// use actix_web::{get, App, HttpRequest, HttpServer, Result};

// #[get("/version/{v1}/{v2}")]
// async fn index(req: HttpRequest) -> Result<String> {
//     let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
//     let v2: u8 = req.match_info().query("v2").parse().unwrap();

//     let (v3, v4): (u8, u8) = req.match_info().load().unwrap();

//     Ok(format!(
//         "Value v1: {}, v2: {}, v3: {}, v4: {}",
//         v1, v2, v3, v4
//     ))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }
// end match_info

// start
// use actix_web::{get, web, App, HttpServer, Result};
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }

// #[get("/{username}/index.html")]
// async fn index(info: web::Path<Info>) -> Result<String> {
//     Ok(format!("Welcome {}!", info.username))
// }

// #[get("/{username}/{id}/index.html")]
// async fn index2(info: web::Path<(String, u32)>) -> Result<String> {
//     let info = info.into_inner();
//     Ok(format!("Welcome {}! id: {}", info.0, info.1))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index).service(index2))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }
// end

// start url generation
// #[get("/test/")]
// async fn index(req: HttpRequest) -> Result<HttpResponse> {
//     let url = req.url_for("foo", ["1", "2", "3"])?;

//     Ok(HttpResponse::Found()
//         .insert_header((header::LOCATION, url.as_str()))
//         .finish())
// }

// use actix_web::{get, guard, http::header, HttpRequest, HttpResponse, Result};
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};

//     HttpServer::new(|| {
//         App::new()
//             .service(
//                 web::resource("/test/{a}/{b}/{c}")
//                     .name("foo")
//                     .guard(guard::Get())
//                     .to(HttpResponse::Ok),
//             )
//             .service(index)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

// #[get("/")]
// async fn index(req: HttpRequest) -> impl Responder {
//     let url = req.url_for("youtube", ["enten "]).unwrap();

//     url.to_string()
// }

// #[get("/show", name = "show_users")]
// async fn show_users(_req: HttpRequest) -> HttpResponse {
//     HttpResponse::Ok().body("Show users")
// }

// async fn generate_url(req: HttpRequest) -> impl Responder {
//     let url = req.url_for("show_users", &[""]);

//     match url {
//         Ok(url) => HttpResponse::Ok().body(format!("generate URL: {}", url)),
//         Err(_) => HttpResponse::InternalServerError().body("Failed to generate URL"),
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};

//     HttpServer::new(|| {
//         App::new()
//             .service(index)
//             .external_resource("youtube", "https://youtube.com/watch/{video_id}")
//             .service(web::scope("/users").service(show_users))
//             .route("/generate", web::get().to(generate_url))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// end url generation

// start path normalization

// use actix_web::{get, http::Method, middleware, HttpResponse};

// #[get("/resource")]
// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("hello")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};

//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::NormalizePath::trim())
//             .service(index)
//             .default_service(web::route().method(Method::GET))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// end path normalization

// start custom guard

use actix_web::{
    guard::{self, Guard, GuardContext},
    http, HttpResponse,
};

struct ContetTypeHeader;

impl Guard for ContetTypeHeader {
    fn check(&self, req: &GuardContext<'_>) -> bool {
        req.head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::route().guard(ContetTypeHeader).to(index))
            .route(
                "/notallowed",
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// end custom guard
