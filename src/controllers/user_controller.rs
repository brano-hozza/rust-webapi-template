
use actix_web::{get, web, HttpResponse, post};
use actix_web::web::ServiceConfig;
use crate::app_state::AppState;
use crate::dtos::user::SignupUserDTO;
use crate::utils::error::AppError;

pub fn routes(cfg: &mut ServiceConfig)  {
    cfg
        .service(get_all_users)
        .service(create_user);
}

#[get("/")]
async fn get_all_users(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    match data.di_container.user_service.get_all_users() {
        Ok(users) =>  Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            println!("Error: {}", err);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[post("/")]
async fn create_user(data: web::Data<AppState>, user_dto: web::Json<SignupUserDTO>) -> Result<HttpResponse, AppError> {
    match data.di_container.user_service.sign_up(user_dto) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            println!("Error: {}", err);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}