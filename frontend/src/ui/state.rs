use crate::be::{load_ui_state, save_ui_state};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use shlib::{AppError, domain::model::UserAccount};

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

impl UiState {
    pub async fn load() -> Result<Self, AppError> {
        load_ui_state().await.map_err(|e: ServerFnError| {
            error!(">>> [UiState::load] Failed to load ui state: {}", e);
            AppError::InternalErr(e.to_string())
        })
    }

    pub async fn save(&self) {
        debug!(">>> [UiState::save] Saving ui state '{:#?}' ...", self);
        if let Err(e) = save_ui_state(self.clone()).await {
            error!(">>> [UiState::save] Failed to save ui state: '{}'.", e);
        };
    }
}
