use crate::ui::{Route, STATE, UiState};
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
                div { class: "bg-[#e2e2e7] dark:bg-[#1e222d] rounded-lg p-6",
                    div { class: "text-xl mb-6 px-2 text-center text-gray-600", "Login to your account" }
                    div { class: "mt-4 space-y-4",
                        div {
                            input {
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-300",
                                name: "email",
                                r#type: "email",
                                placeholder: "Email address",
                                value: "{email}",
                                autofocus: "true",
                                oninput: move |evt| {
                                    email.set(evt.value());
                                },
                                                        // onmounted: move |evt| async move {
                            //     _ = evt.set_focus(true).await;
                            // },
                            }
                        }
                        div {
                            input {
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-300",
                                name: "password",
                                r#type: "password",
                                placeholder: "Password",
                                value: "{password}",
                                oninput: move |e| {
                                    password.set(e.value());
                                },
                                onkeypress: move |evt| {
                                    async move {
                                        if evt.key() == Key::Enter {
                                            handle_login(email(), password(), &mut wrong_creds, &nav).await;
                                        }
                                    }
                                },
                            }
                        }
                        div { class: "text-center text-red-600 my-8",
                            span { class: if !wrong_creds() { "hidden" }, "Wrong credentials" }
                        }
                        div { class: "text-center my-8",
                            button {
                                class: "drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| {
                                    async move {
                                        handle_login(email(), password(), &mut wrong_creds, &nav).await;
                                    }
                                },
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
                    // Persist the state to local store.
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
