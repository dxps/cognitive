use dioxus::prelude::*;
use shlib::domain::model::UserAccount;

/// The global state of the UI.
pub static STATE: GlobalSignal<UiState> = GlobalSignal::new(|| UiState::default());

#[derive(Clone, Default)]
pub struct UiState {
    pub is_ready: bool,
    pub is_light_theme: bool,
    pub user: Option<UserAccount>,
}
