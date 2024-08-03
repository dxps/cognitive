use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
    domain::model::{AttributeDef, Tag},
    server::fns::get_attribute_defs,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_GLOBAL_SIGNALS,
    },
};

#[component]
pub fn AttributeDefListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<AttributeDef>>(|| vec![]);

    let mut tags = use_signal(|| Arc::new(vec![]));

    use_future(move || async move {
        tags.set(UI_GLOBAL_SIGNALS.get_tags().await);
    });

    // TODO: This is not as efficient as `use_server_future`, at least in this case.
    //  See https://dioxuslabs.com/learn/0.5/reference/fullstack/server_functions for details.
    use_future(move || async move {
        if let Ok(attr_defs) = get_attribute_defs().await {
            log::debug!(">>> Got from get_attribute_defs(): {:?}", attr_defs);
            entries.set(attr_defs);
        }
    });

    // TODO: To be considered. Currently, if you do multiple refreshes (F5) on the page, a never ending JS loop is triggered.
    // let res = use_server_future(get_attribute_defs)?().unwrap();
    // if let Ok(data) = res {
    //     entries.set(data);
    // }

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]",
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
                        "The following table lists the existing attributes definitions."
                    }
                    Table { rows: entries(), tags: tags() }
                }
            }
        }
    }
}

#[component]
fn Table(props: TableProps) -> Element {
    //
    let _th_key = "#theader";
    rsx! {
        div { class: "px-6",
            table { class: "min-w-96 bg-white",
                tr { key: "{_th_key}", class: "pr-2 text-left text-sm font-normal text-gray-500",
                    th { class: "min-w-32 pr-2", "name" }
                    th { class: "min-w-64 px-2", "description" }
                    th { class: "min-w-32 px-2", "value type" }
                    th { class: "pl-2", "tag" }
                }
                for attr in props.rows {
                    tr { key: "{attr.id}", class: "p-2 text-left text-sm text-gray-600",
                        td { class: "pr-2",
                            Link {
                                to: Route::AttributeDefEditPage {
                                    attr_def_id: attr.id,
                                },
                                "{attr.name}"
                            }
                        }
                        td { class: "px-2",
                            if attr.description.is_some() {
                                {attr.description.unwrap()}
                            } else {
                                "-"
                            }
                        }
                        td { class: "px-2", "{attr.value_type}" }
                        td { class: "pl-2",
                            { if attr.tag_id.is_some() {
                                let tag_id = attr.tag_id.unwrap();
                                log::debug!(">>> tag_id: {}", tag_id);
                                match props.tags.iter().find(|tag| tag.id == tag_id) {
                                    Some(tag) => {
                                        log::debug!(">>> tag: {:?}", tag);
                                        tag.name.clone()
                                    }
                                    None => {
                                        log::error!(">>> Failed to find tag with id: {}", tag_id);
                                        tag_id
                                    }
                                }
                            }
                            else {
                                attr.tag_id.unwrap_or_default()}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TableProps {
    pub rows: Vec<AttributeDef>,
    pub tags: Arc<Vec<Tag>>,
}
