use crate::ui::{Route, STATE, components::Card};
use dioxus::prelude::*;
use shlib::domain::model::UserAccount;

#[component]
pub fn DataView() -> Element {
    //
    let user_account = STATE.read().user.clone().unwrap_or_else(|| {
        use_navigator().push(Route::HomeView {});
        UserAccount::default()
    });
    if !user_account.is_admin() {
        use_navigator().push(Route::HomeView {});
    }

    rsx! {
        Card {
            header: rsx! {
                h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item) mb-6",
                    "Data"
                }
            },
            content: rsx! {
                div { class: "flex",
                    div { class: "pr-3 flex flex-col grow mr-1",
                        h6 { class: "px-4 mb-2 pt-2 pb-1 block font-medium leading-snug tracking-normal fg-item dark:dark-fg-item antialiased",
                            "Templates"
                        }
                        Link {
                            class: "py-2 px-4 rounded-lg transition duration-200",
                            to: Route::HomeView {},
                            "Objects"
                        }
                        Link {
                            class: "py-2 px-4 rounded-lg transition duration-200",
                            to: Route::HomeView {},
                            "Object Links"
                        }
                        Link {
                            class: "py-2 px-4 rounded-lg transition duration-200",
                            to: Route::AttributeTemplatesListView {
                            },
                            "Attributes"
                        }
                    }
                    div { class: "pr-3 flex flex-col grow ml-1",
                        h6 { class: "px-4 mb-2 pt-2 pb-1 block font-medium leading-snug tracking-normal fg-item dark:dark-fg-item antialiased",
                            "Items"
                        }
                        Link {
                            class: "py-2 px-4 rounded-lg transition duration-200",
                            to: Route::HomeView {},
                            "Objects"
                        }
                        Link {
                            class: "py-2 px-4 rounded-lg transition duration-200",
                            to: Route::HomeView {},
                            "Object Links"
                        }
                    }
                }
            },
        }
    }
}
