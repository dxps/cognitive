use dioxus::prelude::*;

use crate::{
    domain::model::AttributeDef,
    server::fns::list_attribute_defs,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
    },
};

#[component]
pub fn AttributeDefsPage() -> Element {
    //
    let mut entries = use_signal::<Vec<AttributeDef>>(|| vec![]);
    use_future(move || async move {
        if let Ok(attr_defs) = list_attribute_defs().await {
            log::debug!(">>> Received from list_attribute_defs: {:?}", attr_defs);
            entries.set(attr_defs);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefsPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]",
                    div { class: "p-6",
                        h5 { class: "mb-2 block text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                            "Attributes Definitions"
                        }
                        "The following table lists the attributes definitions."
                    }
                    Table { rows: entries() }
                }
            }
        }
    }
}

#[component]
fn Table(props: TableProps) -> Element {
    //
    let th_key = "#theader";
    rsx! {
        div { class: "px-6",
            table { class: "min-w-96 bg-white",
                tr { key: "{th_key}", class: "pr-2 text-left text-sm font-normal text-gray-500",
                    th { class: "min-w-16 pr-2", "name" }
                    th { class: "min-w-32 px-2", "description" }
                    th { "value type" }
                    th { "tag" }
                }
                for attr in props.rows {
                    tr { key: "{attr.id}", class: "p-2 text-left text-sm text-gray-600",
                        td { class: "min-w-16 pr-2",
                            Link { to: Route::AttributeDefsPage {}, "{attr.name}" }
                        }
                        td { class: "min-w-32 px-2",
                            if attr.description.is_some() {
                                {attr.description.unwrap()}
                            } else {
                                "-"
                            }
                        }
                        td { "{attr.value_type}" }
                        td { {attr.tag.unwrap_or_default().name} }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TableProps {
    pub rows: Vec<AttributeDef>,
}
