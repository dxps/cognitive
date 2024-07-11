#![allow(non_snake_case)]

mod server;
mod ui;

fn main() {
    dioxus_logger::init(tracing::Level::DEBUG).expect("failed to init logger");

    #[cfg(feature = "web")]
    dioxus::launch(crate::ui::App);

    #[cfg(feature = "server")]
    server::start_web_server(crate::ui::App);
}
