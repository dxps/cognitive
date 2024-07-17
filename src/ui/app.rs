use crate::ui::routes::Route;
use dioxus::prelude::*;

pub fn App() -> Element {
    //
    _ = console_log::init_with_level(log::Level::Debug);

    rsx! {
        Router::<Route> {}
    }
}
