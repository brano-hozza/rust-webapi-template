
use actix_web::{get, web, HttpResponse};
use actix_web::web::ServiceConfig;
use crate::app_state::AppState;
use crate::repositories::user_repository::UserRepository;
use crate::utils::error::AppError;

pub fn routes(cfg: &mut ServiceConfig)  {
    cfg.service(get_all_users);
}

#[get("/")]
async fn get_all_users(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(data.di_container.user_repository.find_all()))
}