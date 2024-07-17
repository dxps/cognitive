#![allow(non_snake_case)]

use servicentral::{server, ui};

fn main() {
    //
    #[cfg(feature = "web")]
    dioxus::launch(ui::App);

    #[cfg(feature = "server")]
    server::start_web_server(ui::App);
}
