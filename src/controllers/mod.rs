mod user_controller;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api/v1/users").configure(user_controller::routes));
}
