use std::collections::HashMap;

use crate::{
    domain::model::Id,
    server::fns::list_attribute_defs,
    ui::{
        comps::{Breadcrumb, Nav},
        pages::EntityDefForm,
        routes::Route,
        Action,
    },
};
use dioxus::prelude::*;

pub fn EntityDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let included_attr_defs: Signal<Vec<(Id, String)>> = use_signal(|| vec![]);
    let mut all_attr_defs: Signal<HashMap<Id, String>> = use_signal(|| HashMap::new());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityDefListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                        EntityDefForm {
                            name,
                            description,
                            included_attr_defs,
                            all_attr_defs,
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
                                class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    async move {
                                        if name().is_empty() {
                                            err.set(Some("Name cannot be empty".to_string()));
                                            return;
                                        }
                                        let id = handle_create_ent_def(
                                                name(),
                                                description.clone(),
                                                included_attr_defs(),
                                                saved,
                                                err,
                                            )
                                            .await;
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

async fn fetch_all_attr_defs() -> HashMap<Id, String> {
    //
    let mut entries = HashMap::new();
    if let Ok(attr_defs) = list_attribute_defs().await {
        attr_defs.iter().for_each(|attr_def| {
            entries.insert(attr_def.id.clone(), attr_def.name.clone());
        });
    }
    entries
}

async fn handle_create_ent_def(
    name: String,
    description: Option<String>,
    included_attr_defs: Vec<(Id, String)>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    None
}
