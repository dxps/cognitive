use crate::domain::model::UserAccount;
use serde::{Deserialize, Serialize};

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
