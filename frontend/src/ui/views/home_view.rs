use crate::ui::components::{Echo, Hero};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`.
#[component]
pub fn HomeView() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
