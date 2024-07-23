use crate::ui::{routes::Route, ui_global_state::APP_READY, State};
use dioxus::prelude::*;

pub fn App() -> Element {
    //
    _ = console_log::init_with_level(log::Level::Debug);

    // Unfortunately, using this it fails with "wasm-bindgen: imported JS function that was not marked as `catch` threw an error: root is undefined"
    // let state = State::new().expect("Failed to get access to browser's localstorage!");
    let state = State::default();

    _ = use_context_provider(|| Signal::new(state));

    // Asynchronously loading state from localstorage and notify its value through the global signal.
    use_future(move || async move {
        let mut state = use_context::<Signal<State>>();
        if let Ok(local_state) = State::load_from_localstorage() {
            *state.write() = local_state;
            *APP_READY.write() = true;
        }
    });

    rsx! {
        Router::<Route> {}
    }
}
