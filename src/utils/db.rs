use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use dotenv::dotenv;
use std::env;
use crate::constants::env_key;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str, database_name: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(format!("{}/{}",database_url, database_name));
    Pool::builder().build(manager)
}

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE_URL must be set");
    let database_name = env::var(env_key::DATABASE_NAME).expect("DATABASE_NAME must be set");
    init_pool(&database_url, &database_name).expect("Failed to create pool")
}