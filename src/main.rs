#![allow(non_snake_case)]

mod server;
mod ui;

fn main() {
    //
    #[cfg(feature = "web")]
    dioxus::launch(crate::ui::App);

    #[cfg(feature = "server")]
    server::start_web_server(crate::ui::App);
}
