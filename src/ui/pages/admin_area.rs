use dioxus::prelude::*;

use crate::ui::comps::{Nav, NavProps};

#[component]
pub fn AdminArea() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::users_section() }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3",
                    div { class: "text-xl px-2", "Admin Area" }
                }
            }
        }
    }
}
