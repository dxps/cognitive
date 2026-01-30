use crate::ui::components::Card;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`.
#[component]
pub fn HomeView() -> Element {
    rsx! {
        Card {
            header: rsx! {
                h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)", "Home" }
            },
            content: rsx! {
                div {}
            },
        }
    }
}
