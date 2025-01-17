// Route handlers
use crate::routes::get_tags; // Tag handlers
use crate::routes::{
    add_articles_comments, create_article, delete_articles_by_slug, delete_articles_comments,
    favorite_articles_by_slug, get_articles, get_articles_by_slug, get_articles_comments,
    get_articles_feed, unfavorite_articles_by_slug, update_articles_by_slug,
}; // Article handlers
use crate::routes::{delete, login, register, update}; // User handlers
use crate::routes::{follow_profile, get_profile, unfollow_profile};
use crate::routes::{ping, third_party_api}; // Profile handlers

// OpenAPI Schema
use crate::routes::{
    __path_add_articles_comments, __path_create_article, __path_delete,
    __path_delete_articles_by_slug, __path_delete_articles_comments,
    __path_favorite_articles_by_slug, __path_follow_profile, __path_get_articles,
    __path_get_articles_by_slug, __path_get_articles_comments, __path_get_articles_feed,
    __path_get_profile, __path_get_tags, __path_login, __path_ping, __path_register,
    __path_unfavorite_articles_by_slug, __path_unfollow_profile, __path_update,
    __path_update_articles_by_slug,
}; // Path
use crate::schemas::{
    AddComment, ArticleListResponse, ArticleResponseInner, CreateArticle, UpdateArticle,
    UpdateArticleOuter,
};
use crate::schemas::{ArticleTag, TagsResponse};
use crate::schemas::{Profile, ProfileFollow, ProfileResponse, ProfileResponseInner};
use crate::schemas::{UserDelete, UserLogin, UserRegister, UserUpdate};

use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use sqlx::{postgres::PgPoolOptions, PgPool};

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
        paths(ping,
            // user paths
            register, login, update, delete,
            // Profile
            get_profile, follow_profile, unfollow_profile,
            // Tag
            get_tags,
            // Articles
            get_articles, create_article, get_articles_feed, get_articles_by_slug, update_articles_by_slug, delete_articles_by_slug,
            favorite_articles_by_slug, unfavorite_articles_by_slug,
            get_articles_comments, add_articles_comments, delete_articles_comments),
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
        components(schemas(UserRegister, UserLogin, UserUpdate, UserDelete,
            Profile, ProfileResponse, ProfileResponseInner, ProfileFollow,
            ArticleTag, TagsResponse, CreateArticle, ArticleResponseInner, ArticleListResponse, UpdateArticleOuter,
            UpdateArticle, AddComment),)
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
            .route("/third_party_api", web::get().to(third_party_api))
            .service(
                web::scope("/api/v1")
                    // User routes ---------------------------------------------------------------
                    .service(web::resource("users/register").route(web::post().to(register)))
                    .service(web::resource("users/login").route(web::post().to(login)))
                    .service(web::resource("users/update").route(web::put().to(update)))
                    .service(web::resource("users/delete").route(web::delete().to(delete)))
                    // Profile routes ---------------------------------------------------------------
                    .service(web::resource("profiles/{username}").route(web::get().to(get_profile)))
                    .service(
                        web::resource("profiles/{username}/follow")
                            .route(web::post().to(follow_profile))
                            .route(web::delete().to(unfollow_profile)),
                    )
                    // Article routes ---------------------------------------------------------------
                    .service(web::resource("articles").route(web::get().to(get_articles)))
                    .service(
                        web::resource("articles/{username}").route(web::post().to(create_article)),
                    )
                    .service(
                        web::resource("articles/feed/{username}")
                            .route(web::get().to(get_articles_feed)),
                    )
                    .service(
                        web::resource("articles/data/{slug}")
                            .route(web::get().to(get_articles_by_slug))
                            .route(web::put().to(update_articles_by_slug))
                            .route(web::delete().to(delete_articles_by_slug)),
                    )
                    .service(
                        web::resource("articles/favorite/{slug}")
                            .route(web::post().to(favorite_articles_by_slug))
                            .route(web::delete().to(unfavorite_articles_by_slug)),
                    )
                    .service(
                        web::resource("articles/comments/{slug}")
                            .route(web::get().to(get_articles_comments))
                            .route(web::post().to(add_articles_comments))
                            .route(web::post().to(delete_articles_comments)),
                    )
                    // Tags routes ---------------------------------------------------------------
                    .service(web::resource("tags").route(web::get().to(get_tags))),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
