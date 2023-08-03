extern crate diesel;
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod constants;
mod repositories;
mod services;
mod utils;
mod models;
mod schema;
mod app_state;
mod middlewares;
mod controllers;

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