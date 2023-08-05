
use actix_web::{get, web, HttpResponse, post, delete};
use actix_web::web::{ ServiceConfig};
use crate::app_state::AppState;
use crate::dtos::user::SignupUserDTO;
use crate::utils::error::AppError;

pub fn routes(cfg: &mut ServiceConfig)  {
    cfg
        .service(get_all_users)
        .service(get_user)
        .service(create_user)
        .service(delete_user);
}

#[get("")]
async fn get_all_users(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let users = data.di_container.user_service.get_all_users()?;
    Ok(HttpResponse::Ok().json(users))
}


#[get("/{id}")]
async fn get_user(data: web::Data<AppState>, path: web::Path<(String)>) -> Result<HttpResponse, AppError> {
    let (id) = path.into_inner();
    let user = data.di_container.user_service.get_user(id)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("")]
async fn create_user(data: web::Data<AppState>, user_dto: web::Json<SignupUserDTO>) -> Result<HttpResponse, AppError> {
    let user = data.di_container.user_service.sign_up(user_dto)?;
    Ok(HttpResponse::Created().json(user))
}

#[delete("/{id}")]
async fn delete_user(data: web::Data<AppState>, path: web::Path<(String)>)  -> Result<HttpResponse, AppError> {
    let (id) = path.into_inner();
    data.di_container.user_service.delete_user(id)?;
    Ok(HttpResponse::Ok().finish())
}
