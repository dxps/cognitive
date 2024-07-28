use dioxus::prelude::*;

use crate::{
    domain::model::AttributeDef,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
    },
};

#[component]
pub fn AttributeDefsPage() -> Element {
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
                    Table { rows: vec![] }
                }
            }
        }
    }
}

#[component]
fn Table(props: TableProps) -> Element {
    rsx! {
        div { class: "px-6",
            table { class: "min-w-96 bg-white",
                thead {
                    tr { class: "p-2 text-left text-sm font-normal text-gray-500",
                        th { "name" }
                        th { "description" }
                        th { "value type" }
                        th { "tag" }
                    }
                }
                tbody {
                    tr { class: "p-2 text-left text-sm text-gray-600",
                        for attr in props.rows {
                            td {
                                Link { to: Route::AttributeDefsPage {}, "{attr.name}" }
                            }
                            td {
                                if attr.description.is_some() {
                                    {attr.description.unwrap()}
                                } else {
                                    "-"
                                }
                            }
                            td { "{attr.value_type}" }
                            td {
                                if attr.tag.is_some() {
                                    {attr.tag.unwrap().name}
                                } else {
                                    "-"
                                }
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
}
