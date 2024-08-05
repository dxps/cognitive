use std::{collections::HashMap, sync::Arc};

use crate::{
    domain::model::Tag,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_GLOBAL_SIGNALS,
    },
};
use dioxus::prelude::*;

#[component]
pub fn TagListPage() -> Element {
    //
    let mut tags = use_signal(|| Arc::new(HashMap::new()));

    use_future(move || async move {
        tags.set(UI_GLOBAL_SIGNALS.get_tags().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::TagListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Tags"
                            }
                            Link {
                                class: "text-gray-500 font-semibold hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagNewPage {},
                                "+"
                            }
                        }
                        hr { class: "pb-2" }
                        if tags().is_empty() {
                            p { class: "pb-4", "There are no tags defined." }
                        } else {
                            p { class: "pb-4", "The following tags exist." }
                            for item in tags().values() {
                                TagCard { item: item.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TagCard(item: Tag) -> Element {
    rsx! {
        Link { to: Route::TagEditPage { id: item.id },
            div { class: "flex flex-col p-2 my-3 bg-white rounded border hover:bg-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{item.name}"
                    }
                }
                div { class: "flex justify-between text-gray-600",
                    p { class: "text-xs leading-5 text-gray-600 pt-1",
                        "{item.description.unwrap_or_default()}"
                    }
                }
            }
        }
    }
}
