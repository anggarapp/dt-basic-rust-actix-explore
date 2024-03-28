// Start of web::scope

// use actix_web::{web, App, HttpServer, Responder};

// async fn index() -> impl Responder {
//     "Hello From index.html"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(web::scope("/app").route("/index.html", web::get().to(index)))
//     })
//     .bind(("127.0.0.1", 8989))?
//     .run()
//     .await
// }

// End of web::scope

// Start of web::Data

// use actix_web::{get, web, App, HttpServer};
// struct AppState {
//     app_name: String,
// }

// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name: &String = &data.app_name;
//     format!("Hello {app_name}!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .app_data(web::Data::new(AppState {
//                 app_name: String::from("Ashborn"),
//             }))
//             .service(index)
//     })
//     .bind(("127.0.0.1", 8099))?
//     .run()
//     .await
// }

// End of web::Data

// Start of AppStateWithCounter
// use actix_web::{web, get, App, HttpServer};
// use std::sync::{Mutex, MutexGuard};

// struct AppStateWithCounter {
//     counter: Mutex<i32>,
// }

// async fn index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter: MutexGuard<i32> = data.counter.lock().unwrap();
//     *counter += 1;
//     format!("Request Number is {counter}!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });
//     HttpServer::new(move || {
//         App::new()
//             .app_data(counter.clone())
//             .route("/", web::get().to(index))
//     })
//     .bind(("127.0.0.1", 8099))?
//     .run()
//     .await
// }
// End of AppStateWithCounter

// Start of AppStateWithCounter
// use actix_web::{get, web, App, HttpServer, Responder};

// #[get("/login")]
// async fn login() -> impl Responder {
//     "login"
// }
// #[get("/logout")]
// async fn logout() -> impl Responder {
//     "logout"
// }
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         let scope_users = web::scope("/users").service(login).service(logout);
//         App::new().service(scope_users)
//     })
//     .bind(("127.0.0.1", 8099))?
//     .run()
//     .await
// }
// End of AppStateWithCounter

// Start of guard
// use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Eiy Kiddos")
// }
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             web::scope("/guard")
//                 .guard(guard::Get())
//                 .route("/eiy", web::to(|| manual_hello())),
//         )
//     })
//     .bind(("127.0.0.1", 8099))?
//     .run()
//     .await
// }
// End of guard

// Start of configuration
use actix_web::{web, App, HttpResponse, HttpServer};
use dt_actix_application_builder::{auth::scope_auth, security::scope_security};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(scope_security)
            .service(web::scope("/api").configure(scope_auth))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("youare accessing /") }),
            )
    })
    .bind(("127.0.0.1", 8099))?
    .run()
    .await
}
// End of configuration
