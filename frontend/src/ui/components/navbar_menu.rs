use crate::ui::components::icons::{hamburger_icon, logout_icon, user_icon};
use crate::ui::components::toggle_theme_icon;
use crate::ui::{Route, STATE};
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
/// The pages will be rendered under the outlet inside this component.
#[component]
pub fn NavbarMenu() -> Element {
    //
    let state = STATE.read();
    let mut show_dropdown = use_signal(|| false);
    let mut username = use_signal(|| String::from(""));

    if !state.is_ready {
        return rsx! { "" };
    }

    username.set(state.user.clone().unwrap_or_default().username.clone());

    rsx! {
        div { class: "text-sm text-gray-600 hover:bg-[#e2e2e7]
                      dark:text-[#b2804c] dark:hover:text-orange-400
                      rounded-lg flex flex-col items-end overflow-visible",
            button {
                class: "px-4 py-2 align text-sm outline-none rounded-lg dark:bg-[#0f1116]",
                onclick: move |_| {
                    let curr_val = show_dropdown();
                    *show_dropdown.write() = !curr_val;
                },
                div {
                    class: "justify-center",
                    dangerous_inner_html: hamburger_icon(),
                }
            }
            if show_dropdown() {
                NavbarUserMenuDropdown { username, show_dropdown }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct NavUserDropdownProps {
    username: String,
    show_dropdown: Signal<bool>,
}

fn NavbarUserMenuDropdown(mut props: NavUserDropdownProps) -> Element {
    //
    // let has_admin_perms = use_resource(move || async move { has_admin_permissions().await });
    let show_dropdown = props.show_dropdown;

    rsx! {
        div {
            "style": "width: 100%; height: 1000%; padding: 0; position: absolute; top: 0; left: 0",
            onclick: move |_| {
                *props.show_dropdown.write() = false;
            },
            div { class: "w-20 mt-14 mr-[90px] bg-white dark:bg-[#222532] z-[1000] rounded-lg shadow-2xl float-right",
                div {
                    ul { class: "shadow-2xl py-2 min-w-full w-max rounded-lg
                                border-0 max-h-96 overflow-auto bg-white dark:bg-[#222532]",
                        li { class: "flex items-center text-sm cursor-pointer",
                            Link {
                                class: "py-2.5 px-5 min-w-full w-max min-h-full flex",
                                onclick: move |e: MouseEvent| {
                                    e.stop_propagation();
                                    // The signal must be cloned, so that it can be captured
                                    // inside the closure below and the async task can own it.
                                    let show_dropdown = show_dropdown.clone();
                                    spawn(async move {
                                        toggle_light_dark_theme(show_dropdown).await;
                                    });
                                },
                                to: Route::HomeView {},
                                div {
                                    class: "mr-2",
                                    dangerous_inner_html: toggle_theme_icon(),
                                }
                                "Toggle Theme"
                            }
                        }
                        // li { class: "flex items-center text-[#333] hover:bg-gray-100 hover:text-orange-600 text-sm cursor-pointer",
                        //     Link {
                        //         class: "py-2.5 px-5 min-w-full w-max min-h-full flex text-[#333]",
                        //         to: Route::UserProfilePage {
                        //             username: props.username,
                        //         },
                        //         div {
                        //             class: "mr-3",
                        //             dangerous_inner_html: user_icon(),
                        //         }
                        //         "  My profile"
                        //     }
                        // }
                        // {
                        //     match &*has_admin_perms.read_unchecked() {
                        //         Some(Ok(true)) => rsx! {
                        //             li { class: "flex items-center text-[#333] hover:bg-gray-100 hover:text-orange-600 text-sm cursor-pointer",
                        //                 Link {
                        //                     class: "py-2.5 px-5 min-w-full w-max min-h-full flex text-[#333]",
                        //                     to: Route::AdminView {},
                        //                     div { class: "mr-3", dangerous_inner_html: admin_icon() }
                        //                     "  Admin Area"
                        //                 }
                        //             }
                        //         },
                        //         _ => rsx! {},
                        //     }
                        // }
                        li { class: "px-4 py-2",
                            hr { class: "dark:text-gray-700" }
                        }

                        {
                            if props.username.is_empty() {
                                rsx! {
                                    li { class: "flex items-center text-sm cursor-pointer",
                                        Link {
                                            class: "py-2.5 px-5 min-w-full w-max min-h-full flex",
                                            to: Route::LoginView {},
                                            div { class: "mr-3 mt-0.5", dangerous_inner_html: logout_icon() }
                                            "  Login"
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    li { class: "flex items-center text-sm cursor-pointer",
                                        Link {
                                            class: "py-2.5 px-5 min-w-full w-max min-h-full flex",
                                            to: Route::UserProfileView {},
                                            div { class: "mr-3 mt-0.5", dangerous_inner_html: user_icon() }
                                            "  My profile"
                                        }
                                    }
                                    li { class: "flex items-center text-sm cursor-pointer",
                                        Link {
                                            class: "py-2.5 px-5 min-w-full w-max min-h-full flex",
                                            to: Route::LogoutView {},
                                            div { class: "mr-3 mt-0.5", dangerous_inner_html: logout_icon() }
                                            "  Logout"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn toggle_light_dark_theme(mut show_dropdown: Signal<bool>) {
    let mut state = STATE.write();
    state.is_light_theme = !state.is_light_theme;

    // Apply the change.
    if state.is_light_theme {
        _ = document::eval(&format!("document.documentElement.removeAttribute('class');",));
    } else {
        _ = document::eval(&format!("document.documentElement.setAttribute('class', 'dark');",));
    };
    debug!(
        ">>> [NavbarUserMenuDropdown][toggle_light_dark_theme] {} theme applied.",
        if state.is_light_theme { "light" } else { "dark" }
    );

    // Persist the change (all state) to local store.
    state.save().await;

    show_dropdown.set(false);
}
