use crate::routes::ping;
use crate::routes::{__path_delete, __path_login, __path_ping, __path_register, __path_update};
use crate::routes::{delete, login, register, update}; // User handlers

use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::schemas::{UserDelete, UserLogin, UserRegister, UserUpdate};
use crate::settings::{DatabaseSettings, Settings};

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.database_connection_string())
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build_app(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = start(listener, connection_pool)?;

        Ok(Self { port, server })
    }

    pub async fn run_app(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub fn start(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(ping, register, login, update, delete),
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
        ),
        components(schemas(UserRegister, UserLogin, UserUpdate, UserDelete,),)
    )]

    struct ApiDoc;

    let db_pool_data = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/apidoc/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(db_pool_data.clone())
            .route("/ping", web::get().to(ping))
            .service(
                web::scope("/api/v1")
                    // User routes ---------------------------------------------------------------
                    .service(web::resource("users/register").route(web::post().to(register)))
                    .service(web::resource("users/login").route(web::post().to(login)))
                    .service(web::resource("users/update").route(web::put().to(update)))
                    .service(web::resource("users/delete").route(web::delete().to(delete))),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
