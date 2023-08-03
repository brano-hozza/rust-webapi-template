use crate::constants::env_key;
use actix_cors::Cors;
use std::env;

pub fn cors() -> Cors {
    let frontend_origin = env::var(env_key::FRONTEND_ORIGIN).unwrap_or_else(|_| "*".to_string());

    Cors::default()
        .allowed_origin(&frontend_origin)
        .allow_any_header()
}