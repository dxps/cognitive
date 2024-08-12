use crate::{
    domain::model::AttributeDef,
    server::fns::create_attribute_def,
    ui::{
        comps::{AttributeDefForm, Breadcrumb, Nav},
        routes::Route,
        Action, UI_GLOBALS,
    },
};
use dioxus::prelude::*;
use std::{collections::HashMap, sync::Arc};

#[component]
pub fn AttributeDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let value_type = use_signal(|| "text".to_string());
    let default_value = use_signal(|| "".to_string());
    let is_required = use_signal(|| false);
    let is_multivalued = use_signal(|| false);
    let tag_id = use_signal(|| "".to_string());
    let mut tags = use_signal(|| Arc::new(HashMap::new()));

    let err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        tags.set(UI_GLOBALS.get_tags().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create an Attribute Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                        "Fill in the following form to create a new attribute definition."
                        AttributeDefForm {
                            name,
                            description,
                            value_type,
                            default_value,
                            is_required,
                            is_multivalued,
                            tag_id,
                            tags: tags(),
                            action: Action::Edit
                        }
                        div { class: "flex justify-betweent mt-8",
                            // Show the button's action result in the UI.
                            div { class: "min-w-[440px] max-w-[440px]",
                                if err().is_some() {
                                    span { class: "text-red-600 flex justify-center",
                                        { err().unwrap() }
                                    }
                                } else if saved() {
                                    span { class: "text-green-600 flex justify-center",
                                        { "Successfully created" }
                                    }
                                }
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| {
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    let tag_id = match tag_id().is_empty() {
                                        true => None,
                                        false => Some(tag_id()),
                                    };
                                    async move {
                                        let item = AttributeDef {
                                            id: "".to_string(),
                                            name: name(),
                                            description,
                                            value_type: value_type().into(),
                                            default_value: default_value(),
                                            is_required: is_required(),
                                            is_multivalued: is_multivalued(),
                                            tag_id,
                                        };
                                        create_handler(item, saved, err).await;
                                    }
                                },
                                "Create"
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn create_handler(item: AttributeDef, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    log::debug!("Creating an attribute definition {:?}: ", item);
    match create_attribute_def(item).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
