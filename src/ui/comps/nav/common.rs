use dioxus::prelude::*;

use crate::ui::{comps::Nav, routes::Route};

/// Navigation header specific function to highlight the link based on the current path.
pub fn style_nav_item_link(curr_path: &String, link_path: String) -> &'static str {
    if *curr_path == link_path {
        "text-sm text-green-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200"
    } else {
        "text-sm text-gray-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200"
    }
}

pub fn render_go_to_login() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-10",
                    div { "You need to login to access this page." }
                    div { class: "mt-6 text-center",
                        Link {
                            class: "text-sm text-gray-600 py-2 px-4 hover:bg-gray-50 rounded-lg transition duration-200",
                            to: Route::Login {},
                            "Login"
                        }
                    }
                }
            }
        }
    }
}
