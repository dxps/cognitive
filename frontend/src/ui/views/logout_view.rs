use crate::ui::{Route, STATE};
use dioxus::prelude::*;

#[component]
pub fn LogoutView() -> Element {
    //

    use_effect(|| {
        debug!(">>> [LogoutView] Logging out ...");
        spawn(async move {
            handle_logout().await;
        });
        debug!(">>> [LogoutView] Logout done.");
    });

    rsx! {
        div { class: "pt-[var(--nav-height)] min-h-[calc(100vh-var(--nav-height))] flex",
            div { class: "flex flex-col grow justify-center items-center py-6 drop-shadow-2xl",
                div { class: "bg-white dark:bg-(--dark-bg-d1) rounded-lg p-6 sm:min-w-[600px] sm:min-h-[200px]",
                    div { class: "text-lg mb-6 px-2 font-medium text-center text-(--fg-item) dark:text-(--dark-fg-item)",
                        "Logged out"
                    }
                    div { class: "mt-4 space-y-4",
                        div { class: "flex justify-center rounded-lg p-3",
                            p { class: "px-2", "Have a great day! See you later!" }
                        }
                        div { class: "flex justify-center",
                            Link {
                                class: "text-sm py-2 px-4 rounded-lg transition duration-200",
                                to: Route::HomeView {},
                                "Back to Home"
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn handle_logout() {
    let default_session = "".to_string();
    let mut state = STATE.write();
    let session = state.session.as_ref().unwrap_or(&default_session).as_str();
    log::debug!(">>> [handle_logout] session: {:?}", session);
    match reqwest::Client::new()
        .post("http://localhost:9011/auth/logout")
        .header("Authorization", session)
        .send()
        .await
    {
        Ok(rsp) => {
            match rsp.status() {
                reqwest::StatusCode::NO_CONTENT => {
                    debug!(">>> [handle_logout] Logout successful.");
                    state.session = None;
                    state.user = None;
                    debug!(
                        ">>> [handle_logout] Updated state w/ session: {:?} user: {:?}",
                        state.session, state.user
                    );

                    // Persist the state to local store.
                    state.save().await;
                    debug!(">>> [handle_logout] Logout done.");
                }
                _ => {
                    debug!(">>> [handle_logout] Logout failed. http status code: {}", rsp.status());
                }
            }
        }
        Err(e) => {
            debug!(">>> [handle_logout] Request err: {}", e);
        }
    }
}
