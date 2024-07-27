use dioxus::prelude::*;

use crate::ui::{
    comps::{Breadcrumb, Nav},
    routes::Route,
};

#[component]
pub fn AttributeDefsPage() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefsPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
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
