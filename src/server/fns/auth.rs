use crate::domain::model::{Id, UserAccount};

#[cfg(feature = "server")]
use crate::server::Session;

#[cfg(feature = "server")]
use log::debug;

use dioxus_fullstack::prelude::*;

// TODO: Use a proper result type, instead of `ServerFnError`.
// pub type LoginResult = AppResult<UserAccount>;

#[server(Login)]
pub async fn login(email: String, password: String) -> Result<UserAccount, ServerFnError> {
    //
    let session: Session = extract().await?;
    let account = session.1.authenticate_user(email, password).await?;
    session.login_user(account.id.clone());
    debug!("[login] Logged-in user with account: {:?}", account);
    Ok(account)
}

#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    session.logout_user();
    Ok(())
}

#[server(GetUserName)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    let name = match session.0.current_user {
        Some(user) => user.username,
        None => "".to_string(),
    };
    Ok(name)
}

#[server(Permissions)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    use axum_session_auth::Rights;

    let method: axum::http::Method = extract().await?;
    let session: Session = extract().await?;
    let current_user = session.current_user.clone().unwrap_or_default();

    // Let's check permissions only and not worry about if the user is anonymous or not.
    if !axum_session_auth::Auth::<UserAccount, Id, sqlx::PgPool>::build([axum::http::Method::POST], false)
        .requires(Rights::any([
            Rights::permission("Category::View"),
            Rights::permission("Admin::View"),
        ]))
        .validate(&current_user, &method, None)
        .await
    {
        return Ok(format!(
            "User '{}' does not have permissions needed to view this page. Please login.",
            current_user.username
        ));
    }

    Ok(format!(
        "User '{}' has the needed permissions to view this page. Here are his permissions: {:?}",
        current_user.username, current_user.permissions
    ))
}

#[server(HasAdminPermissions)]
pub async fn has_admin_permissions() -> Result<bool, ServerFnError> {
    use axum_session_auth::Rights;

    let method: axum::http::Method = extract().await?;
    let session: Session = extract().await?;
    let current_user = session.current_user.clone().unwrap_or_default();

    // Let's check permissions only and not worry about if the user is anonymous or not.
    let res = !axum_session_auth::Auth::<UserAccount, Id, sqlx::PgPool>::build([axum::http::Method::POST], false)
        .requires(Rights::any([
            Rights::permission("Admin::Read"),
            Rights::permission("Admin::Write"),
        ]))
        .validate(&current_user, &method, None)
        .await;
    Ok(res)
}
