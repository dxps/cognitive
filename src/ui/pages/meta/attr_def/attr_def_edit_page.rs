use crate::{
    domain::model::AttributeDef,
    server::fns::{get_attribute_def, tags::get_tags, update_attribute_def},
    ui::{
        comps::{AttributeDefForm, Breadcrumb, Nav},
        routes::Route,
    },
};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AttributeDefEditPageProps {
    attr_def_id: String,
}

#[component]
pub fn AttributeDefEditPage(props: AttributeDefEditPageProps) -> Element {
    //
    let mut attr_def = use_signal(|| None);
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut value_type = use_signal(|| "".to_string());
    let mut default_value = use_signal(|| "".to_string());
    let mut is_required = use_signal(|| false);
    let mut is_multivalued = use_signal(|| false);
    let mut tag_id = use_signal(|| "".to_string());
    let mut tags = use_signal(|| vec![]);

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut saved = use_signal(|| false);

    use_future(move || async move {
        tags.set(get_tags().await.unwrap_or_default());
    });
    let id = use_signal(|| props.attr_def_id.clone());
    use_future(move || async move {
        attr_def.set(get_attribute_def(id()).await.unwrap_or_default());
        if attr_def().is_some() {
            let item = attr_def().unwrap();
            name.set(item.name);
            description.set(item.description.unwrap_or_default());
            value_type.set(item.value_type.to_string());
            default_value.set(item.default_value);
            is_required.set(item.is_required);
            is_multivalued.set(item.is_multivalued);
            tag_id.set(item.tag_id.unwrap_or_default());
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb {
                paths: Route::get_path(Route::AttributeDefEditPage {
                    attr_def_id: id(),
                })
            }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Edit an Attribute Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                        "Change any of the fields below to update the attribute definition."
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
                        div { class: "flex justify-between my-8",
                            button {
                                class: "text-red-200 bg-slate-50 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| { async move { todo!() } },
                                "Delete"
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
                                        match update_attribute_def(
                                                AttributeDef::new(
                                                    id(),
                                                    name(),
                                                    description,
                                                    value_type().into(),
                                                    default_value(),
                                                    is_required(),
                                                    is_multivalued(),
                                                    tag_id,
                                                ),
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
                                },
                                "Update"
                            }
                        }
                        // Show the button's action result in the UI.
                        if err().is_some() {
                            div { class: "text-center text-red-600 my-8",
                                span { {err().unwrap()} }
                            }
                        } else if saved() {
                            div { class: "text-center text-green-600 my-8",
                                span { { "Successfully updated" } }
                            }
                        }
                    }
                }
            }
        }
    }
}
