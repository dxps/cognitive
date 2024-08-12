use crate::{
    domain::model::{AttributeDef, Tag},
    server::fns::list_attribute_defs,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_GLOBALS,
    },
};
use dioxus::prelude::*;
use std::{collections::HashMap, sync::Arc};

#[component]
pub fn AttributeDefListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<AttributeDef>>(|| vec![]);

    let mut tags = use_signal(|| Arc::new(HashMap::new()));

    use_future(move || async move {
        tags.set(UI_GLOBALS.get_tags().await);

        if let Ok(attr_defs) = list_attribute_defs().await {
            log::debug!(">>> Got from get_attribute_defs(): {:?}", attr_defs);
            entries.set(attr_defs);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Attributes Definitions"
                            }
                            Link {
                                class: "text-gray-500 font-semibold hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefNewPage {},
                                "+"
                            }
                        }
                        hr { class: "pb-2" }
                        p { class: "pb-4",
                            "The following table lists the existing attributes definitions."
                        }
                        for attr in entries() {
                            AttrDefCard { attr_def: attr.clone(), tags: tags() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttrDefCard(attr_def: AttributeDef, tags: Arc<HashMap<String, Tag>>) -> Element {
    rsx! {
        Link {
            to: Route::AttributeDefPage {
                attr_def_id: attr_def.id,
            },
            div { class: "flex flex-col p-2 my-3 bg-white rounded border hover:bg-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{attr_def.name}"
                    }
                    p { class: "text-xs text-slate-500 leading-snug tracking-normal antialiased pr-1",
                        "{attr_def.value_type.label()}"
                    }
                }
                div { class: "flex justify-between text-gray-600",
                    p { class: "text-xs leading-5 text-gray-600 pt-1",
                        "{attr_def.description.unwrap_or_default()}"
                    }
                    {   if attr_def.tag_id.is_some() {
                        let tag_id = attr_def.tag_id.unwrap();
                        log::debug!(">>> tag_id: {}", tag_id);
                        match tags.get(&tag_id) {
                                Some(tag) => {
                                    log::debug!(">>> tag: {:?}", tag);
                                    rsx! { p { class: "text-xs leading-5 bg-slate-100 rounded-lg px-2", {tag.name.clone()} } }
                                }
                                None => {
                                    log::error!(">>> Failed to find tag with id: {}", tag_id);
                                    rsx! {}
                                }
                            }
                        }
                        else {
                            rsx! { p { {attr_def.tag_id.unwrap_or_default()} } }
                        }
                    }
                }
            }
        }
    }
}
