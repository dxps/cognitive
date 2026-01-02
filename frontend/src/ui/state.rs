use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use shlib::{AppError, domain::model::UserAccount};

use crate::be::{load_ui_state, save_ui_state};

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
            println!("Failed to load ui state: {}", e);
            AppError::InternalErr(e.to_string())
        })
    }

    pub async fn save(&self) {
        if let Err(e) = save_ui_state(self.clone()).await {
            error!("Failed to save ui state: {}", e);
        };
    }
}
