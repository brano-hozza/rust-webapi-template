use std::str::FromStr;

use uuid::Uuid;

use crate::dtos::user::{SignupUserDTO, UpdateUserDTO};
use crate::models::user::{UpdateUser, User};
use crate::repositories::user_repository::{UserRepository, UserRepositoryImpl};
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: UserRepositoryImpl,
}

impl UserServiceImpl {
    pub fn new(user_repository: UserRepositoryImpl) -> Self {
        Self { user_repository }
    }
    pub fn sign_up(&self, user_dto: SignupUserDTO) -> Result<User, AppError> {
        self.user_repository.create(
            user_dto.email.as_str(),
            user_dto.username.as_str(),
            user_dto.password.as_str(),
        )
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.user_repository.find_all()
    }
    pub fn get_user(&self, id: String) -> Result<User, AppError> {
        self.user_repository.find(Uuid::from_str(id.as_str()).unwrap())
    }
    pub fn update_user(&self, id: String, user: UpdateUserDTO) -> Result<User, AppError> {
        self.user_repository.update(Uuid::from_str(id.as_str()).unwrap(), UpdateUser::new(user))
    }
    pub fn delete_user(&self, id: String) -> Result<(), AppError> {
        self.user_repository.delete(Uuid::from_str(id.as_str()).unwrap())
    }
}