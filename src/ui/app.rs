use crate::{
    server::fns::config::get_ui_config,
    ui::{routes::Route, ui_global_state::APP_READY, UiState},
};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    //
    _ = console_log::init_with_level(log::Level::Debug);

    // Unfortunately, using this it fails with "wasm-bindgen: imported JS function that was not marked as `catch` threw an error: root is undefined"
    // let state = State::new().expect("Failed to get access to browser's localstorage!");
    let state = UiState::default();

    _ = use_context_provider(|| Signal::new(state));

    // Asynchronously loading state from localstorage and notify its value through the global signal.
    use_future(move || async move {
        let mut state = use_context::<Signal<UiState>>();
        if let Ok(mut local_state) = UiState::load_from_localstorage() {
            local_state.logo = state().logo;
            *state.write() = local_state;
            *APP_READY.write() = true;
        }
    });

    // Fetch the UI config.
    use_future(move || async move {
        let ui_config = get_ui_config().await;
        log::debug!(">>> [App] Got UI config: {:?}", ui_config);
        if ui_config.is_ok() {
            let mut state = use_context::<Signal<UiState>>();
            *state.write() = UiState {
                logo: ui_config.unwrap().logo_path,
                ..state().clone()
            };
        }
    });

    rsx! {
        Router::<Route> {}
    }
}
