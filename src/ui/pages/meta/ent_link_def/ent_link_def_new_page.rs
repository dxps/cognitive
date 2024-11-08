use dioxus::prelude::*;

use crate::{
    domain::model::{EntityDef, EntityLinkDef},
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_STATE,
    },
};

#[component]
pub fn EntityLinkDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());

    let mut ent_defs = use_signal::<Vec<EntityDef>>(|| Vec::new());

    use_future(move || async move {
        ent_defs.set(UI_STATE.get_ent_defs_list().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityLinkDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity Link Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkDefListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-4" }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityLinkDefCard(item: EntityLinkDef) -> Element {
    rsx! {
        Link {
            to: Route::EntityDefPage {
                id: item.id,
            },
            div { class: "flex flex-col p-2 my-3 bg-white rounded-lg border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
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
