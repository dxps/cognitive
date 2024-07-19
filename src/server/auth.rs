use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::PgPool;
use std::sync::Arc;

use crate::server::user_mgmt::UserMgmt;
use crate::server::{ServerState, UserAccount, UsersRepo};

#[async_trait]
impl Authentication<UserAccount, String, PgPool> for UserAccount {
    async fn load_user(user_id: String, pool: Option<&PgPool>) -> Result<UserAccount, anyhow::Error> {
        let pool = pool.unwrap();
        UsersRepo::get_by_id(user_id, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.is_anonymous
    }

    fn is_active(&self) -> bool {
        !self.is_anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }
}

#[async_trait]
impl HasPermission<PgPool> for UserAccount {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.permissions.contains(&perm.to_string())
    }
}

pub struct Session(
    /// auth session
    pub AuthSession<UserAccount, String, SessionPgPool, PgPool>,
    /// user management
    pub Arc<UserMgmt>,
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

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSession layer was not found!")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSession layer was not found!",
        )
            .into_response()
    }
}

#[async_trait]
impl<S: Sync + Send> axum::extract::FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(parts: &mut http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        AuthSession::<UserAccount, String, SessionPgPool, PgPool>::from_request_parts(parts, state)
            .await
            .map(|auth_session| {
                let ss = parts.extensions.get::<ServerState>().unwrap();
                let am = ss.auth_mgr.clone();
                Session(auth_session, am)
            })
            .map_err(|_| AuthSessionLayerNotFound)
    }
}
