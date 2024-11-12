use dioxus::prelude::*;

use crate::{
    domain::model::EntityLink,
    server::fns::list_entity_links,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
    },
};

#[component]
pub fn EntityLinkListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<EntityLink>>(|| vec![]);

    use_future(move || async move {
        match list_entity_links().await {
            Ok(items) => entries.set(items),
            Err(e) => {
                // TODO: Capture the error and display it.
                log::error!("Failed to list entity links: {}", e)
            }
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityLinkDefListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Entity Link Definitions"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkDefNewPage {},
                                "+"
                            }
                        }
                        if entries.is_empty() {
                            p { class: "pb-4 text-gray-500", "There are no entries." }
                        }
                        for item in entries() {
                            EntityLinkCard { item: item.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityLinkCard(item: EntityLink) -> Element {
    //
    rsx! {
        Link {
            to: Route::EntityLinkDefPage {
                id: item.id,
            },
            div { class: "flex flex-col px-3 py-2 my-3 bg-white rounded-lg border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{item.kind}"
                    }
                }
                div {
                    pre { class: "text-xs leading-5 text-gray-600 pt-1", " " }
                }
            }
        }
    }
}
