use crate::utils::db::DbPool;

use crate::repositories::user_repository::UserRepositoryImpl;
use crate::services::user_service::UserServiceImpl;

#[derive(Clone)]
pub struct DiContainer {
    /**
     * User
     */
    pub user_repository: UserRepositoryImpl,
    pub user_service: UserServiceImpl,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let user_repository = UserRepositoryImpl::new(pool.clone());
        let user_service = UserServiceImpl::new(user_repository.clone());

        Self {
            // User
            user_repository,
            user_service,
        }
    }
}
