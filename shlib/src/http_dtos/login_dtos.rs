use serde::{Deserialize, Serialize};

use crate::domain::model::UserAccount;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub session: String,
    pub expires_in_seconds: i64,
    pub user: Option<UserAccount>,
}
