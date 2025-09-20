use crate::handler;
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api").service(
            web::scope("auth")
                .route("/register", web::post().to(handler::auth::register))
                .route("/login", web::post().to(handler::auth::login)),
        ),
    );
}
