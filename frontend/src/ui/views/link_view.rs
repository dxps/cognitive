use crate::ui::{Route, STATE};
use dioxus::prelude::*;
use shlib::domain::model::{Id, UserAccount};

#[component]
pub fn LinkView(id: Id) -> Element {
    //
    let user_account = STATE.read().user.clone().unwrap_or_else(|| {
        use_navigator().push(Route::LoginView {});
        UserAccount::default()
    });

    rsx! {
        div { class: "flex flex-col min-h-screen",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-(--bg-d1) dark:bg-(--dark-bg-d1) rounded-lg p-6 sm:min-w-[600px] sm:min-h-[600px]",
                    h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)",
                        "Link View"
                    }
                }
            }
        }
    }
}
