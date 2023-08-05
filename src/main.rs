extern crate diesel;
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod app_state;
mod constants;
mod controllers;
mod dtos;
mod middlewares;
mod models;
mod repositories;
mod schema;
mod services;
mod utils;

extern crate dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start conduit server...");
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        use crate::app_state::AppState;
        AppState::new(pool)
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(middlewares::cors::cors())
            .configure(controllers::api)
    })
    .bind(constants::BIND)?
    .run()
    .await
}
