use diesel::{debug_query, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use diesel::pg::Pg;
use crate::utils::db::DbPool;
use crate::models::user::{SignupUser, User};
use crate::schema::users;
use crate::utils::error::AppError;
use crate::utils::hasher;

pub trait UserRepository: Send + Sync + 'static {
    fn create<'a>(
        &self,
        email: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Result<User, AppError>;
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
    fn create<'a>(
        &self,
        email: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Result<User, AppError> {
        let conn = &mut self.pool.get()?;
        let hashed_password = hasher::hash_password(password)?;

        let record = SignupUser {
            email,
            username,
            password: &hashed_password,
        };

        let query = diesel::insert_into(users::table)
            .values(&record);

        let user = query.get_result::<User>(conn)?;
        Ok(user)
    }

    fn find_all(&self) -> Result<Vec<User>, AppError> {
        let conn = &mut self.pool.get()?;
        let query = users::table.select(User::as_select());
        let users = query.get_results(conn)?;
        Ok(users)
    }
}