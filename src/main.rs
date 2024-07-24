#![allow(non_snake_case)]

use dioxus::prelude::VNode;
use servicentral::ui::{App, AppProps};
#[cfg(feature = "server")]
use servicentral::{server, ui};

#[cfg(feature = "web")]
#[cfg(not(feature = "server"))]
use servicentral::ui;

type ElementFn = fn() -> std::option::Option<VNode>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    #[cfg(feature = "server")]
    dotenvy::dotenv()?;

    let ui_launcher: ElementFn = app_fn;

    #[cfg(feature = "web")]
    dioxus::launch(ui_launcher);

    #[cfg(feature = "server")]
    // server::start_web_server(ui::App);
    server::start_web_server(ui_launcher);

    Ok(())
}

fn app_fn() -> std::option::Option<VNode> {
    let logo_path = std::env::var("LOGO_PATH").ok();
    log::debug!(">>> [app_fn] logo_path={:?}", logo_path);
    let props = AppProps { logo_path };
    App(props)
}
