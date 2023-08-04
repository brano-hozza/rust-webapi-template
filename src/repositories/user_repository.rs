use diesel::{debug_query, QueryDsl, RunQueryDsl, SelectableHelper};
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

        let user = match query.get_result::<User>(conn) {
            Ok(user) => {user}
            Err(err) => {
                println!("Err:{}\nQuery:{}",err, debug_query::<Pg, _>(&query).to_string());
                panic!();
            }
        };

        Ok(user)
    }

    fn find_all(&self) -> Result<Vec<User>, AppError> {
        let conn = &mut self.pool.get()?;
        let t = users::table.select(User::as_select());
        let users = match t.get_results(conn).ok() {
            None => Vec::<User>::new(),
            Some(users) => users
        };
        Ok(users)
    }
}