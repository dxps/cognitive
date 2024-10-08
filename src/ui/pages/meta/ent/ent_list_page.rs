use dioxus::prelude::*;

use crate::{
    domain::model::Entity,
    ui::{
        comps::{Breadcrumb, Modal, Nav},
        routes::Route,
        UI_GLOBALS,
    },
};

#[component]
pub fn EntityListPage() -> Element {
    //
    let mut items = use_signal::<Vec<Entity>>(|| vec![]);
    let mut show_modal = use_signal(|| false);

    use_future(move || async move {
        UI_GLOBALS.get_ent_kinds().await;
    });

    // use_future(move || async move {
    //     if let Ok(ent_defs) = list_entities_defs().await {
    //         items.set(ent_defs);
    //     }
    // });

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
        if show_modal() {
            Modal {
                title: "Create Entity",
                content: "Click on the kind of entity you want to create.",
                children: rsx! {
                    Link { to: Route::EntityNewPage {}, "Kind X" }
                }
            }
        }
    }
}

#[component]
fn EntityCard(ent: Entity) -> Element {
    rsx! {
        Link { to: Route::EntityPage { id: ent.id },
            div { class: "flex flex-col p-2 my-3 bg-white rounded border hover:bg-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{ent.kind}"
                    }
                }
                div { class: "flex justify-between text-gray-600",
                    p { class: "text-xs leading-5 text-gray-600 pt-1", "" }
                }
            }
        }
    }
}
