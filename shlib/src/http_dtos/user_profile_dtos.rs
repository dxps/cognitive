use serde::{Deserialize, Serialize};

use crate::domain::model::{Id, UserAccount};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileUpdateRequest {
    pub id: Id,
    pub email: String,
    pub username: String,
    pub bio: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPasswordUpdateRequest {
    pub id: Id,
    pub curr_password: String,
    pub new_password: String,
}

impl Into<UserAccount> for UserProfileUpdateRequest {
    fn into(self) -> UserAccount {
        UserAccount {
            id: self.id,
            email: self.email,
            username: self.username,
            bio: self.bio,
            is_anonymous: false,
            permissions: [].to_vec(),
        }
    }
}
