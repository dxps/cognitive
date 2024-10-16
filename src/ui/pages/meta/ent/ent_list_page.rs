use dioxus::prelude::*;

use crate::{
    domain::model::Entity,
    server::fns::list_entities,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_GLOBALS,
    },
};

#[component]
pub fn EntityListPage() -> Element {
    //
    let mut items = use_signal::<Vec<Entity>>(|| vec![]);

    use_future(move || async move {
        UI_GLOBALS.get_ent_defs().await;
        if let Ok(entitites) = list_entities().await {
            items.set(entitites);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Entities"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityNewPage {},
                                "+"
                            }
                        }
                        hr { class: "pb-2" }
                        p { class: "pb-4",
                            if items.is_empty() {
                                "No entities exist."
                            } else {
                                "The following entities exist."
                            }
                        }
                        for item in items() {
                            EntityCard { ent: item.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityCard(ent: Entity) -> Element {
    //
    log::debug!("[EntityCard] ent: {:?}", ent);
    rsx! {
        Link {
            to: Route::EntityPage {
                id: ent.id.clone(),
            },
            div { class: "flex flex-col p-2 my-3 bg-white rounded border hover:bg-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    div { class: "flex ",
                        p { class: "text-sm leading-5 text-gray-400 pt-1 pl-2",
                            "{ent.listing_attr_name}:"
                        }
                        p { class: "text-sm leading-5 text-gray-800 font-medium pt-1 pl-2",
                            "{ent.listing_attr_value}"
                        }
                    }
                    p { class: "mt-1 text-xs bg-slate-100 hover:bg-white rounded-lg px-2 leading-snug tracking-normal antialiased",
                        "{ent.kind}"
                    }
                }
            }
        }
    }
}
