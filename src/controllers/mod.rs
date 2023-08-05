use actix_web::web;
use actix_web::web::ServiceConfig;

mod user_controller;

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1").service(
                web::scope("/users").configure(user_controller::routes)
            )
        )
    );
}
