use actix_web::{get, web, App, HttpServer};

// Start of web::scope
// async fn index() -> impl Responder {
//     "Hello From index.html"
// }
// End of web::scope
// Start of web::Data
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name: &String = &data.app_name;
    format!("Hello {app_name}!")
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
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Ashborn"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8099))?
    .run()
    .await
    // End of web::Data
}
