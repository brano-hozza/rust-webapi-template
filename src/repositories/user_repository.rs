use crate::utils::db::DbPool;
use uuid::Uuid;
use crate::models::user::{UpdateUser, User};
use crate::utils::error::AppError;

pub trait UserRepository: Send + Sync + 'static {
    fn signup(
        &self,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<User, AppError>;
    fn update(&self, user_id: Uuid, changeset: UpdateUser) -> Result<User, AppError>;
    fn find(&self, user_id: Uuid) -> Result<User, AppError>;
    fn find_all(&self) -> Result<Vec<User>, AppError>;
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn signup(
        &self,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<User, AppError> {
        let conn = &mut self.pool.get()?;
        User::signup(conn, email, username, password)
    }


    fn update(&self, user_id: Uuid, changeset: UpdateUser) -> Result<User, AppError> {
        let conn = &mut self.pool.get()?;
        let new_user = User::update(conn, user_id, changeset)?;
        Ok(new_user)
    }

    fn find(&self, user_id: Uuid) -> Result<User, AppError> {
        let conn = &mut self.pool.get()?;
        let user = User::find(conn, user_id)?;
        Ok(user)
    }

    fn find_all(&self) -> Result<Vec<User>, AppError> {
        let conn = &mut self.pool.get()?;
        let users = User::find_all(conn)?;
        Ok(users)
    }
}