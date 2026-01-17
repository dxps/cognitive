use crate::domain::model::{ADMIN_READ_PERMISSION, ADMIN_WRITE_PERMISSION, Id};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// User account contains most of the details of a user (except password related ones).
#[derive(Debug, Default, Clone, PartialEq, Props, Serialize, Deserialize, ToSchema)]
pub struct UserAccount {
    pub id: Id,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub is_anonymous: bool,
    pub permissions: Vec<String>,
}

impl UserAccount {
    pub fn is_admin(&self) -> bool {
        self.permissions
            .iter()
            .all(|p| p.starts_with(ADMIN_READ_PERMISSION) || p.starts_with(ADMIN_WRITE_PERMISSION))
    }
}

#[derive(Debug)]
/// It includes all user attributes that are persisted in the database.
pub struct UserEntry {
    pub user: UserAccount,
    pub password: String,
    pub salt: String,
}

impl From<UserEntry> for UserAccount {
    fn from(entry: UserEntry) -> Self {
        entry.user
    }
}

/// It includes just the user's password and salt.
pub struct UserPasswordSalt {
    pub password: String,
    pub salt: String,
}
