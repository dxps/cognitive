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

use super::{AttributeDefMgmt, AttributeDefRepo, TagMgmt, TagsRepo, UserMgmt, UsersRepo};

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct ServerState {
    pub user_mgmt: Arc<UserMgmt>,
    pub tag_mgmt: Arc<TagMgmt>,
    pub attr_def_mgmt: Arc<AttributeDefMgmt>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        //
        let users_repo = Arc::new(UsersRepo::new(db_pool.clone()));
        let user_mgmt = Arc::new(UserMgmt::new(users_repo));

        let tag_repo = Arc::new(TagsRepo::new(db_pool.clone()));
        let tag_mgmt = Arc::new(TagMgmt::new(tag_repo));

        let attr_def_repo = Arc::new(AttributeDefRepo::new(db_pool.clone()));
        let attr_def_mgmt = Arc::new(AttributeDefMgmt::new(attr_def_repo));

        Self {
            user_mgmt,
            tag_mgmt,
            attr_def_mgmt,
        }
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
