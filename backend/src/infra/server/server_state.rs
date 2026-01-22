use crate::{
    domain::logic::{AttributeTemplateMgmt, UserMgmt},
    infra::{AttributeTemplateRepo, UserRepo},
};
use axum::extract::{FromRef, FromRequestParts};
use http::{StatusCode, request::Parts};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub user_mgmt: Arc<UserMgmt>,
    pub attr_tmpl_mgmt: Arc<AttributeTemplateMgmt>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        //
        let users_repo = Arc::new(UserRepo::new(db_pool.clone()));
        let user_mgmt = Arc::new(UserMgmt::new(users_repo));
        let attr_tmpl_repo = Arc::new(AttributeTemplateRepo::new(db_pool.clone()));
        let attr_tmpl_mgmt = Arc::new(AttributeTemplateMgmt::new(attr_tmpl_repo));

        Self { user_mgmt, attr_tmpl_mgmt }
    }
}

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
