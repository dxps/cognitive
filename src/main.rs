#![allow(non_snake_case)]

#[cfg(feature = "server")]
use akasha::{server, ui};

#[cfg(feature = "web")]
#[cfg(not(feature = "server"))]
use akasha::ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    #[cfg(feature = "server")]
    dotenvy::dotenv()?;

    #[cfg(feature = "web")]
    dioxus::launch(ui::App);

    #[cfg(feature = "server")]
    server::start_web_server(ui::App);

    Ok(())
}
