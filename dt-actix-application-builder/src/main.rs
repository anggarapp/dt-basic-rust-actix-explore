use actix_web::{web, App, HttpServer};
use std::sync::{Mutex, MutexGuard};

// Start of web::scope
// async fn index() -> impl Responder {
//     "Hello From index.html"
// }
// End of web::scope
// Start of web::Data
// struct AppState {
//     app_name: String,
// }

// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name: &String = &data.app_name;
//     format!("Hello {app_name}!")
// }
// End of web::Data
// Start of web::Data
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter: MutexGuard<i32> = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request Number is {counter}!")
}
// End of web::Data

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start of web::scope
    // HttpServer::new(|| {
    //     App::new().service(web::scope("/app").route("/index.html", web::get().to(index)))
    // })
    // .bind(("127.0.0.1", 8989))?
    // .run()
    // .await
    // End of web::scope

    // Start of web::Data
    // HttpServer::new(|| {
    //     App::new()
    //         .app_data(web::Data::new(AppState {
    //             app_name: String::from("Ashborn"),
    //         }))
    //         .service(index)
    // })
    // .bind(("127.0.0.1", 8099))?
    // .run()
    // .await
    // End of web::Data
    // Start of web::Data
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8099))?
    .run()
    .await
    // End of web::Data
}
