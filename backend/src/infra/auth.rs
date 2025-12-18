use crate::infra::UserRepo;
use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session_auth::*;
use shlib::domain::model::{Id, UserAccount};
use sqlx::PgPool;

pub const SESSION_NAME: &str = "Authorization";
pub const SESSION_TABLE: &str = "user_sessions";

// ---------------------------------
//          AuthUserAccount
// ---------------------------------

/// To mitigate the orphan rule, we implement the traits for UserAccount here.
/// Both `UserAccount` and `Authentication` are defined outside of this crate.
#[derive(Clone, Debug)]
pub struct AuthUserAccount(pub UserAccount);

impl From<UserAccount> for AuthUserAccount {
    fn from(user: UserAccount) -> Self {
        AuthUserAccount(user)
    }
}

#[async_trait]
impl Authentication<AuthUserAccount, Id, PgPool> for AuthUserAccount {
    async fn load_user(
        user_id: Id,
        pool: Option<&PgPool>,
    ) -> Result<AuthUserAccount, anyhow::Error> {
        let pool = pool.unwrap();
        UserRepo::get_by_id(&user_id, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.0.is_anonymous
    }

    fn is_active(&self) -> bool {
        !self.0.is_anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.0.is_anonymous
    }
}

#[async_trait]
impl HasPermission<PgPool> for AuthUserAccount {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.0.permissions.iter().any(|p| p == perm)
    }
}

// ---------------------------------
//     AuthSessionLayerNotFound
// ---------------------------------

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
