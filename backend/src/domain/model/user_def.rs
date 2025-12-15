use shlib::domain::model::{Id, UserAccount};

use crate::domain::model::new_id;

pub fn new_default_user_account() -> UserAccount {
    UserAccount {
        id: new_id(),
        is_anonymous: true,
        username: "Guest".into(),
        email: "".into(),
        bio: "".into(),
        permissions: Vec::new(),
    }
}
