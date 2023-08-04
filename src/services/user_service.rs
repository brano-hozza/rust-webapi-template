use actix_web::web::Json;
use crate::dtos::user::SignupUserDTO;
use crate::models::user::User;
use crate::repositories::user_repository::{UserRepository, UserRepositoryImpl};
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct UserServiceImpl{
    user_repository: UserRepositoryImpl
}

impl UserServiceImpl{
    pub fn new(user_repository: UserRepositoryImpl) -> Self {
        Self { user_repository }
    }
    pub fn sign_up(&self, user_dto: Json<SignupUserDTO>) -> Result<User, AppError>{
        self.user_repository.create(
            user_dto.email.as_str(),
            user_dto.username.as_str(),
            user_dto.password.as_str()
        )
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, AppError>{
        self.user_repository.find_all()
    }
}