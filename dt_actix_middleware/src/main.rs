// start using middleware
use actix_web::{dev::Service, get, middleware, App, HttpResponse, HttpServer};
use futures_util::future::FutureExt;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // middleware 1
            .wrap(middleware::Compress::default()) // middleware 2
            .wrap(middleware::NormalizePath::trim()) // middleware 3
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());

                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .service(index)
    })
    .bind(("127.0.0.1", 8099))?
    .run()
    .await
}
// end using middleware
