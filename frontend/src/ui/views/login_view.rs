use crate::ui::{Route, STATE};
use dioxus::{prelude::*, router::Navigator};
use shlib::http_dtos::{LoginRequest, LoginResponse};

#[component]
pub fn LoginView() -> Element {
    //
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut wrong_creds = use_signal(|| false);
    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col min-h-screen",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-(--bg-d1) dark:bg-(--dark-bg-d1) rounded-lg p-6 sm:min-w-[600px] sm:min-h-[200px]",
                    div { class: "text-xl mb-6 px-2 text-center text-(--fg-item) dark:text-(--dark-fg-item)",
                        "Login to your account"
                    }
                    form {
                        class: "mt-4 space-y-4",
                        onsubmit: move |e| {
                            e.prevent_default();
                            async move {
                                handle_login(email(), password(), &mut wrong_creds, &nav).await;
                            }
                        },
                        div {
                            label {
                                class: "text-sm text-gray-500 block mb-2",
                                r#for: "email",
                                "Email"
                            }
                            input {
                                class: "px-3 py-2 rounded-lg outline-none",
                                id: "email",
                                r#type: "email",
                                value: "{email}",
                                autocomplete: "email",
                                oninput: move |evt| {
                                    email.set(evt.value());
                                },
                                onmounted: move |evt| async move {
                                    // UX: Focus the email input.
                                    _ = evt.set_focus(true).await;
                                },
                            }
                        }
                        div {
                            label {
                                class: "text-sm text-gray-500 block mb-2",
                                r#for: "password",
                                "Password"
                            }
                            input {
                                class: "px-3 py-2 rounded-lg outline-none",
                                id: "password",
                                r#type: "password",
                                value: "{password}",
                                autocomplete: false,
                                oninput: move |e| {
                                    password.set(e.value());
                                },
                            }
                        }
                        div { class: "text-center text-red-600 my-8",
                            span { class: if !wrong_creds() { "hidden" }, "Wrong credentials" }
                        }
                        div { class: "text-center my-8",
                            button {
                                class: "drop-shadow-sm px-4 py-2 rounded-md",
                                r#type: "submit",
                                name: "loginBtn",
                                "Login"
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn handle_login(email: String, password: String, wrong_creds: &mut Signal<bool>, nav: &Navigator) {
    let mut state = STATE.write();

    let input = LoginRequest {
        email: email.clone(),
        password: password.clone(),
    };

    match reqwest::Client::new()
        .post("http://localhost:9011/auth/login")
        .json(&input)
        .send()
        .await
    {
        Ok(rsp) => {
            match rsp.json::<LoginResponse>().await {
                Ok(rsp) => {
                    log::debug!(">>> [handle_login] rsp: {:#?}", rsp);
                    wrong_creds.set(false);
                    state.session = Some(rsp.session);
                    state.user = rsp.user;
                    // Persist the state to local store:
                    // - on Web, it's the front-end's dedicated service (not the back-end).
                    // - on Mobile, it's also the front-end's dedicated service that runs on the mobile device.
                    state.save().await;
                    nav.push(Route::HomeView {});
                }
                Err(e) => {
                    log::debug!(">>> [handle_login] Authentication failed. Error: {}", e);
                    if e.to_string().contains("wrong credentials") {
                        wrong_creds.set(true);
                    }
                }
            };
        }
        Err(e) => {
            log::debug!(">>> [handle_login] request err: {}", e);
        }
    }
}
