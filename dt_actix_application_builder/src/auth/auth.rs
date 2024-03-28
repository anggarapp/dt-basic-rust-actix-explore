use actix_web::{web, HttpResponse};

pub fn scope_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth")
            .route(
                web::get().to(|| async { HttpResponse::Ok().body("you are accesing /api/auth") }),
            )
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
