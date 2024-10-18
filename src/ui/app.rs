use crate::ui::{routes::Route, ui_globals::UI_GLOBALS, UiStorage};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    //
    _ = console_log::init_with_level(log::Level::Debug);

    // Unfortunately, using this it fails with "wasm-bindgen: imported JS function that was not marked as `catch` threw an error: root is undefined"
    // let state = State::new().expect("Failed to get access to browser's localstorage!");
    let state = UiStorage::default();

    _ = use_context_provider(|| Signal::new(state));

    // Asynchronously loading state from localstorage and notify its value through the global signal `app_ready`.
    use_future(move || async move {
        let mut state = use_context::<Signal<UiStorage>>();
        if let Ok(local_state) = UiStorage::load_from_localstorage() {
            *state.write() = local_state;
            *UI_GLOBALS.app_ready.write() = true;
        }
    });

    rsx! {
        Router::<Route> {}
    }
}
