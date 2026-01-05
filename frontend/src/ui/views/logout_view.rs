use dioxus::prelude::*;

use crate::ui::{Route, STATE};

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
        div { class: "flex flex-col min-h-screen",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-[#e2e2e7] dark:bg-[#1e222d] rounded-lg p-6",
                    div { class: "text-lg mb-6 px-2 text-center text-gray-600", "Logged out" }
                    div { class: "mt-4 space-y-4",
                        div { class: "rounded-lg p-3",
                            div { class: "px-2", "Have a great day! See you later!" }
                        }
                        div { class: "pt-8",
                            Link { to: Route::HomeView {}, "Back to Home" }
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
