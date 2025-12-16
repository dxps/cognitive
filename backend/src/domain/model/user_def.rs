use shlib::domain::model::UserAccount;

use crate::domain::model::new_id;

/// Creates a default anonymous (`Guest` username) user account.
/// Created as an alternative to the `Default` trait implementation.
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
