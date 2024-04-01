use crate::routes::__path_ping;
use crate::routes::ping;

use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn start(listener: TcpListener) -> Result<Server, std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(ping),
        info(
            title = "Actix but CRUD Again",
            version = "0.1.0",
            description = "if CRUD Again, so what?",
            license(name = "MIT", url = "github.com"),
            contact(
                name = "Maidenless Tarnished",
                url = "github.com",
                email = "maidenless@tarnished.com"
            )
        )
    )]

    struct ApiDoc;

    let server = HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/apidoc/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/ping", web::get().to(ping))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
