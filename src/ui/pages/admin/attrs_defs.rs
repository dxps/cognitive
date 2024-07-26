use dioxus::prelude::*;

use crate::ui::{
    comps::{Breadcrumb, Nav, NavProps},
    routes::Route,
};

#[component]
pub fn AttributeDefsPage() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::users_section() }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                Breadcrumb {
                    paths: vec![
                        ("Home".into(), Route::Home {}),
                        ("Admin".into(), Route::Admin {}),
                        ("Attributes Definitions".into(), Route::AttributeDefsPage {}),
                    ]
                }
                div { class: "bg-white rounded-md p-3 min-w-96",
                    div { class: "p-6",
                        h5 { class: "mb-2 block text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                            "Attributes Definitions"
                        }
                    }
                }
            }
        }
    }
}
