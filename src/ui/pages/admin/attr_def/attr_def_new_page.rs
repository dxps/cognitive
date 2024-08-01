use dioxus::prelude::*;

use crate::{
    domain::model::{AttributeDef, Tag},
    server::fns::{create_attribute_def, tags::get_tags},
    ui::{
        comps::{AttributeDefForm, Breadcrumb, Nav},
        routes::Route,
    },
};

struct CreateAttributeDef {
    name: String,
    description: String,
    value_type: String,
    default_value: String,
    is_required: bool,
    is_multivalued: bool,
    tag_id: String,
}

#[component]
pub fn AttributeDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let value_type = use_signal(|| "".to_string());
    let default_value = use_signal(|| "".to_string());
    let is_required = use_signal(|| false);
    let is_multivalued = use_signal(|| false);
    let tag_id = use_signal(|| "".to_string());
    let mut tags = use_signal(|| vec![]);

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut saved = use_signal(|| false);

    use_future(move || async move {
        tags.set(get_tags().await.unwrap_or_default());
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]",
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
                            tags
                        }
                        div { class: "text-center my-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| {
                                    async move {
                                        let input = CreateAttributeDef {
                                            name: name(),
                                            description: description(),
                                            value_type: value_type(),
                                            default_value: default_value(),
                                            is_required: is_required(),
                                            is_multivalued: is_multivalued(),
                                            tag_id: tag_id(),
                                        };
                                        create_handler(input, saved, err).await;
                                    }
                                },
                                "Create"
                            }
                        }
                        // Show the button's action result in the UI.
                        if err().is_some() {
                            div { class: "text-center text-red-600 my-8",
                                span { {err().unwrap()} }
                            }
                        } else if saved() {
                            div { class: "text-center text-green-600 my-8",
                                span { { "Successfully created" } }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn create_handler(input: CreateAttributeDef, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    match create_attribute_def(
        input.name,
        input.description,
        input.value_type,
        input.default_value,
        input.is_required,
        input.is_multivalued,
        input.tag_id,
    )
    .await
    {
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
