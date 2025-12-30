use crate::ui::components::icons::{hamburger_icon, logout_icon, user_icon};
use crate::ui::{Route, STATE, UiState, UiStorage};
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
        div { class: "text-sm text-gray-600 hover:bg-[#e2e2e7] dark:hover:bg-[#1e222d] rounded-lg flex flex-col items-end overflow-visible",
            button {
                class: "px-4 py-2 align text-sm outline-none rounded-lg",
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

    rsx! {
        div {
            "style": "width: 100%; height: 1000%; padding: 0; position: absolute; top: 0; left: 0",
            onclick: move |_| {
                *props.show_dropdown.write() = false;
            },
            div { class: "w-20 mt-14 mr-[64px] bg-white dark:bg-[#222532] z-[1000] rounded-lg shadow-2xl float-right",
                div {
                    ul { class: "shadow-2xl bg-white dark:bg-[#222532] py-2 min-w-full w-max rounded-lg dark:rounded-lg border-0 max-h-96 overflow-auto",
                        li { class: "flex items-center text-sm cursor-pointer",
                            Link {
                                class: "py-2.5 px-5 min-w-full w-max min-h-full flex",
                                onclick: |e: MouseEvent| {
                                    e.stop_propagation();
                                    debug!(">>> [NavbarUserMenuDropdown] Toggle theme.");
                                    toggle_light_dark_theme();
                                },
                                to: Route::HomeView {},
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
                            hr {}
                        }

                        {
                            if props.username.is_empty() {
                                rsx! {
                                    li { class: "flex items-center text-sm cursor-pointer",
                                        Link {
                                            class: "py-2.5 px-5 min-w-full w-max min-h-full flex",
                                            to: Route::LoginView {},
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
                                            div { class: "mr-3", dangerous_inner_html: user_icon() }
                                            "  My profile"
                                        }
                                    }
                                    li { class: "flex items-center text-sm cursor-pointer",
                                        Link {
                                            class: "py-2.5 px-5 min-w-full w-max min-h-full flex",
                                            to: Route::LogoutView {},
                                            div { class: "mr-3", dangerous_inner_html: logout_icon() }
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

fn toggle_light_dark_theme() {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.document_element().unwrap();
    let mut state = STATE.write();
    state.is_light_theme = !state.is_light_theme;
    debug!(
        ">>> [toggle_light_dark_theme] is_light_theme: {}",
        state.is_light_theme
    );
    if state.is_light_theme {
        root.class_list().remove_1("dark").unwrap();
    } else {
        root.class_list().add_1("dark").unwrap();
    }
    // Persist the state to localstorage.
    let mut storage: UiStorage<UiState> = UiStorage::new("cognitive_state").unwrap_or_default();
    storage.data = Some(state.clone());
    storage.save_to_localstorage();
}
