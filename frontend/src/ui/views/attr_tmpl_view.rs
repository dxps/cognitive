use crate::ui::{Route, STATE, components::Card};
use dioxus::prelude::*;
use shlib::domain::model::{Id, UserAccount};

#[component]
pub fn AttributeTemplateView(id: Id) -> Element {
    //
    let user_account = STATE.read().user.clone().unwrap_or_else(|| {
        use_navigator().push(Route::LoginView {});
        UserAccount::default()
    });
    if !user_account.is_admin() {
        use_navigator().push(Route::HomeView {});
    }

    rsx! {
        Card {
            header: rsx! {
                h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)", "Attribute Template" }
            },
            content: rsx! {
                div { class: "mt-4", "id: {id}" }
            },
        }
    }
}
