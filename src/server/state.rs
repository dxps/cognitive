use std::sync::Arc;

#[cfg(feature = "server")]
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
#[cfg(feature = "server")]
use http::{request::Parts, StatusCode};
#[cfg(feature = "server")]
use sqlx::PgPool;

use super::{user_mgmt::UserMgmt, UsersRepo};

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct ServerState {
    pub auth_mgr: Arc<UserMgmt>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        let users_repo = Arc::new(UsersRepo::new(db_pool.clone()));
        let auth_mgr = Arc::new(UserMgmt::new(users_repo));
        Self { auth_mgr }
    }
}

#[cfg(feature = "server")]
#[async_trait]
impl<S> FromRequestParts<S> for ServerState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
