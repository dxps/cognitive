use std::sync::Arc;

use sqlx::PgPool;

use crate::{domain::logic::UserMgmt, infra::UserRepo};

#[derive(Clone)]
pub struct ServerState {
    pub user_mgmt: Arc<UserMgmt>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        //
        let users_repo = Arc::new(UserRepo::new(db_pool.clone()));
        let user_mgmt = Arc::new(UserMgmt::new(users_repo));

        Self { user_mgmt }
    }
}
