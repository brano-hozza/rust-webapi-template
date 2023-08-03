use crate::utils::db::DbPool;

use crate::repositories::user_repository::UserRepositoryImpl;

#[derive(Clone)]
pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: UserRepositoryImpl,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let user_repository = UserRepositoryImpl::new(pool.clone());

        Self {
            // User
            user_repository,
        }
    }
}