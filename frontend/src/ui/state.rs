use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use shlib::domain::model::UserAccount;

/// The global state of the UI.
pub static STATE: GlobalSignal<UiState> = GlobalSignal::new(|| UiState::default());

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UiState {
    //
    #[serde(skip)]
    pub is_ready: bool,

    pub is_light_theme: bool,

    pub session: Option<String>,

    pub user: Option<UserAccount>,
}
