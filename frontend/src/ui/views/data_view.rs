use crate::ui::{Route, STATE};
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
        div { class: "pt-[var(--nav-height)] min-h-[calc(100vh-var(--nav-height))] flex",
            div { class: "flex flex-col grow justify-center items-center py-6 drop-shadow-2xl",
                div { class: "bg-white dark:bg-(--dark-bg-d1) rounded-lg p-4 sm:min-w-[600px] sm:min-h-[500px]",
                    h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item) mb-6",
                        "Data"
                    }
                    div { class: "flex",
                        div { class: "pr-3 flex flex-col grow mr-1",
                            h6 { class: "px-4 mb-2 pt-2 pb-1 block font-medium leading-snug tracking-normal fg-item dark:dark-fg-item antialiased",
                                "Templates"
                            }
                            Link {
                                class: "py-2 px-4 rounded-lg transition duration-200",
                                to: Route::HomeView {},
                                "Entities"
                            }
                            Link {
                                class: "py-2 px-4 rounded-lg transition duration-200",
                                to: Route::HomeView {},
                                "Entity Links"
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
                                "Instances"
                            }
                            Link {
                                class: "py-2 px-4 rounded-lg transition duration-200",
                                to: Route::HomeView {},
                                "Entities"
                            }
                            Link {
                                class: "py-2 px-4 rounded-lg transition duration-200",
                                to: Route::HomeView {},
                                "Entity Links"
                            }
                        }
                    }
                }
            }
        }
    }
}
