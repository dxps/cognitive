use dioxus::prelude::*;

use crate::{
    domain::model::EntityDef,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
    },
};

#[component]
pub fn EntityDefListPage() -> Element {
    //
    let mut items = use_signal::<Vec<EntityDef>>(|| vec![]);

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityDefListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Entities Definitions"
                            }
                            Link {
                                class: "text-gray-500 font-semibold hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityDefNewPage {},
                                "+"
                            }
                        }
                        hr { class: "pb-2" }
                        p { class: "pb-4",
                            if items.is_empty() {
                                "There are no entities definitions."
                            } else {
                                "The following entities definitions are defined."
                            }
                        }
                        for item in items() {
                            EntityDefCard { ent_def: item.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityDefCard(ent_def: EntityDef) -> Element {
    rsx! {
        Link {
            to: Route::EntityDefPage {
                id: ent_def.id,
            },
            div { class: "flex flex-col p-2 my-3 bg-white rounded border hover:bg-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{ent_def.name}"
                    }
                }
                div { class: "flex justify-between text-gray-600",
                    p { class: "text-xs leading-5 text-gray-600 pt-1",
                        "{ent_def.description.unwrap_or_default()}"
                    }
                }
            }
        }
    }
}
