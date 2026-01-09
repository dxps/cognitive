use crate::ui::{Route, STATE};
use dioxus::prelude::*;
use shlib::{
    AppError, AppResult,
    domain::model::{Id, UserAccount},
    http_dtos::{ErrorResponse, UserPasswordUpdateRequest, UserProfileUpdateRequest},
};

const CSS_TAB_ACTIVE: &str = "rounded-lg font-semibold text-center text-sm py-2 px-4 tracking-wide cursor-pointer";
const CSS_TAB_INACTIVE: &str = "rounded-lg text-center text-sm hover:bg-white hover:text-lilac py-2 px-4 tracking-wide cursor-pointer";

#[component]
pub fn UserProfileView() -> Element {
    //
    let user_account = STATE.read().user.clone().unwrap_or_else(|| {
        use_navigator().push(Route::LoginView {});
        UserAccount::default()
    });
    let mut tab_to_show = use_signal(|| "primary_info".to_string());

    rsx! {
        div { class: "flex flex-col min-h-screen",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-(--bg-d1) dark:bg-(--dark-bg-d1) rounded-lg p-6 sm:min-w-[600px] sm:min-h-[650px]",
                    h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)",
                        {format!("{}'s Profile", user_account.username)}
                    }
                    // The tabs.
                    ul { class: "flex gap-4 bg-gray-100 rounded-lg my-4 px-[3.4px] w-max overflow-hidden font-sans mx-auto",
                        li {
                            class: if tab_to_show() == "primary_info".to_string() { CSS_TAB_ACTIVE } else { CSS_TAB_INACTIVE },
                            onclick: move |_| tab_to_show.set("primary_info".to_string()),
                            "Primary Info"
                        }
                        li {
                            class: if tab_to_show() == "security".to_string() { CSS_TAB_ACTIVE } else { CSS_TAB_INACTIVE },
                            onclick: move |_| tab_to_show.set("security".to_string()),
                            "Security"
                        }
                    }
                    if tab_to_show() == "primary_info".to_string() {
                        PrimaryInfo { user_account }
                    } else if tab_to_show() == "security".to_string() {
                        Security { user_account }
                    }
                }
            }
        }
    }
}

#[component]
fn PrimaryInfo(user_account: UserAccount) -> Element {
    //
    let mut username = use_signal(|| user_account.username.clone());
    let mut email = use_signal(|| user_account.email.clone());
    let mut bio = use_signal(|| user_account.bio.clone());
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut saved = use_signal(|| false);

    rsx! {
        div { class: "mt-8 space-y-6",
            div {
                label {
                    class: "text-sm text-gray-500 block mb-2",
                    r#for: "username",
                    "Username"
                }
                input {
                    class: "w-full",
                    r#type: "text",
                    id: "username",
                    placeholder: "Username",
                    value: "{user_account.username}",
                    autocomplete: false,
                    maxlength: 48,
                    oninput: move |evt| { username.set(evt.value()) },
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", r#for: "email", "Email" }
                input {
                    class: "w-full rounded-md py-2.5",
                    r#type: "text",
                    id: "email",
                    placeholder: "Email",
                    value: "{user_account.email}",
                    autocomplete: false,
                    maxlength: 64,
                    oninput: move |evt| { email.set(evt.value()) },
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", r#for: "bio", "Biography" }
                textarea {
                    class: "w-full rounded-md py-2.5 px-3",
                    cols: 64,
                    rows: 6,
                    id: "bio",
                    placeholder: "Biography",
                    value: "{user_account.bio}",
                    maxlength: 1024,
                    oninput: move |evt| { bio.set(evt.value()) },
                }
            }
            div { class: "text-center my-8",
                button {
                    onclick: move |_| {
                        let mut ua = user_account.clone();
                        async move {
                            match update_user_profile(ua.id.clone(), username(), email(), bio()).await {
                                Ok(_) => {
                                    err.set(None);
                                    saved.set(true);
                                    ua.username = username();
                                    ua.email = email();
                                    ua.bio = bio();
                                    let mut state = STATE.write();
                                    state.user = Some(ua);
                                    state.save().await;
                                }
                                Err(e) => {
                                    err.set(Some(e.to_string()));
                                    saved.set(false);
                                }
                            }
                        }
                    },
                    "Update"
                }
            }
            // Show the result in the UI.
            if err().is_some() {
                div { class: "text-center text-red-600 my-8",
                    span { {err().unwrap()} }
                }
            } else if saved() {
                div { class: "text-center text-green-600 my-8",
                    span { {"Successfully updated"} }
                }
            }
        }
    }
}

#[component]
fn Security(user_account: UserAccount) -> Element {
    //
    let mut curr_password = use_signal(|| String::new());
    let mut new_password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let mut result_err: Signal<Option<String>> = use_signal(|| None);
    let mut result_ok = use_signal(|| false);

    rsx! {
        div { class: "mt-8 space-y-6",
            div { class: "flex flex-row text-sm",
                {"Id: "}
                {user_account.id.to_string()}
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Current Password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Enter the current password",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { curr_password.set(evt.value()) },
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "New Password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Enter the new password",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { new_password.set(evt.value()) },
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Confirm New Password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Enter the new password again",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { confirm_password.set(evt.value()) },
                }
            }
            div { class: "text-center my-8",
                button {
                    onclick: move |_| {
                        let ua = user_account.clone();
                        async move {
                            log::debug!(
                                ">>> [Security] Received: curr_password: {}, new_password: {}, confirm_password: {}",
                                curr_password(), new_password(), confirm_password()
                            );
                            if new_password().is_empty() || new_password() != confirm_password() {
                                result_err
                                    .set(
                                        Some(
                                            "The new password and confirm password do not match.".into(),
                                        ),
                                    );
                                return;
                            }
                            match update_user_password(ua.id.clone(), curr_password(), new_password())
                                .await
                            {
                                Ok(()) => {
                                    result_err.set(None);
                                    result_ok.set(true);
                                }
                                Err(e) => {
                                    result_err.set(Some(e.to_string()));
                                }
                            }
                        }
                    },
                    "Update"
                }
            }
            // Show the result in the UI.
            if result_err().is_some() {
                div { class: "text-center text-red-600 my-8",
                    span { {result_err().unwrap()} }
                }
            } else if result_ok() {
                div { class: "text-center text-green-600 my-8",
                    span { {"Successfully updated"} }
                }
            }
        }
    }
}

async fn update_user_profile(id: Id, username: String, email: String, bio: String) -> AppResult<()> {
    //
    let input = UserProfileUpdateRequest { id, email, username, bio };

    match reqwest::Client::new()
        .put("http://localhost:9011/user/profile")
        .json(&input)
        .send()
        .await
    {
        Ok(rsp) => match rsp.status() {
            reqwest::StatusCode::NO_CONTENT => {
                log::debug!(">>> [update_user_profile] Successfully updated.");
                Ok(())
            }
            _ => {
                log::debug!(">>> [update_user_profile] Update failed. http status code: {}", rsp.status());
                Err(AppError::Err(rsp.status().to_string()))
            }
        },
        Err(e) => {
            log::debug!(">>> [update_user_profile] Request error: {}", e);
            Err(AppError::Err(e.to_string()))
        }
    }
}

async fn update_user_password(id: Id, curr_password: String, new_password: String) -> AppResult<()> {
    //
    let input = UserPasswordUpdateRequest {
        id,
        curr_password,
        new_password,
    };

    match reqwest::Client::new()
        .put("http://localhost:9011/user/password")
        .json(&input)
        .send()
        .await
    {
        Ok(rsp) => match rsp.status() {
            reqwest::StatusCode::NO_CONTENT => {
                log::debug!(">>> [update_user_profile] Successfully updated.");
                Ok(())
            }
            _ => {
                log::debug!(">>> [update_user_profile] Update failed. http status code: {}", rsp.status());
                match rsp.json::<ErrorResponse>().await {
                    Ok(err_rsp) => Err(AppError::Err(err_rsp.error)),
                    Err(err) => {
                        error!("[update_user_password] Error: {}", err);
                        Err(AppError::InternalErr("Internal error".into()))
                    }
                }
            }
        },
        Err(e) => {
            log::debug!(">>> [update_user_profile] Request error: {}", e);
            Err(AppError::Err(e.to_string()))
        }
    }
}
