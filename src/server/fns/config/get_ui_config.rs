use super::UiConfig;
use dioxus_fullstack::prelude::*;

#[server(GetUiConfigServerData)]
pub async fn get_ui_config() -> Result<UiConfig, ServerFnError> {
    let logo_path = std::env::var("LOGO_PATH").ok();
    Ok(UiConfig { logo_path })
}
