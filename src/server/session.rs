use std::sync::Arc;

use axum::async_trait;
use axum_session_auth::AuthSession;
use axum_session_sqlx::SessionPgPool;
use sqlx::PgPool;

use crate::domain::model::UserAccount;

use super::{AttributeDefMgmt, AuthSessionLayerNotFound, EntityDefMgmt, EntityMgmt, ServerState, TagMgmt, UserMgmt};

pub struct Session(
    //
    pub AuthSession<UserAccount, String, SessionPgPool, PgPool>,
    pub Arc<UserMgmt>,
    pub Arc<TagMgmt>,
    pub Arc<AttributeDefMgmt>,
    pub Arc<EntityDefMgmt>,
    pub Arc<EntityMgmt>,
);

impl std::ops::Deref for Session {
    type Target = AuthSession<UserAccount, String, SessionPgPool, PgPool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<S: Sync + Send> axum::extract::FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(parts: &mut http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        AuthSession::<UserAccount, String, SessionPgPool, PgPool>::from_request_parts(parts, state)
            .await
            .map(|auth_session| {
                let server_state = parts.extensions.get::<ServerState>().unwrap();
                let user_mgmt = server_state.user_mgmt.clone();
                let tag_mgmt = server_state.tag_mgmt.clone();
                let attr_def_mgmt = server_state.attr_def_mgmt.clone();
                let ent_def_mgmt = server_state.ent_def_mgmt.clone();
                let ent_mgmt = server_state.ent_mgmt.clone();
                Session(auth_session, user_mgmt, tag_mgmt, attr_def_mgmt, ent_def_mgmt, ent_mgmt)
            })
            .map_err(|_| AuthSessionLayerNotFound)
    }
}
