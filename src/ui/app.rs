use crate::ui::{routes::Route, ui_global_state::APP_READY, UiState};
use dioxus::prelude::*;

#[component]
pub fn App(props: AppProps) -> Element {
    //
    _ = console_log::init_with_level(log::Level::Debug);

    // Unfortunately, using this it fails with "wasm-bindgen: imported JS function that was not marked as `catch` threw an error: root is undefined"
    // let state = State::new().expect("Failed to get access to browser's localstorage!");
    log::debug!(">>> [App] Got {:?}.", props);
    let state = match props.logo_path {
        Some(logo_path) => UiState::new_with_logo(logo_path),
        None => UiState::default(),
    };

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

    rsx! {
        Router::<Route> {}
    }
}

#[derive(Debug, PartialEq, Props, Clone)]
pub struct AppProps {
    pub logo_path: Option<String>,
}
