use std::collections::HashMap;

use crate::{
    domain::model::{EntityDef, Id},
    ui::{
        comps::{Breadcrumb, Nav},
        pages::{meta::ent_def::fetch_all_attr_defs, EntityDefForm},
        routes::Route,
        Action,
    },
};
use dioxus::prelude::*;

pub fn EntityDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let included_attr_defs = use_signal::<Vec<(Id, String)>>(|| vec![]);
    let mut listing_attr_def_id = use_signal(|| Id::default());

    let mut all_attr_defs = use_signal(|| HashMap::<Id, String>::new());

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
                            listing_attr_def_id,
                            all_attr_defs,
                            action: Action::Edit,
                            saved,
                            err
                        }
                        div { class: "flex justify-betweent mt-8",
                            // Show the button's action result in the UI.
                            div { class: "min-w-[450px] max-w-[450px] text-sm flex justify-center items-center",
                                if err().is_some() {
                                    span { class: "text-red-600", { err().unwrap() } }
                                } else if saved() {
                                    span { class: "text-green-600", { "Successfully created" } }
                                }
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: included_attr_defs().is_empty(),
                                onclick: move |_| {
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    async move {
                                        if saved() {
                                            navigator().push(Route::EntityDefListPage {});
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            if included_attr_defs().is_empty() {
                                                err.set(Some("Include at least one attribute".to_string()));
                                                return;
                                            }
                                            let attributes_ids: Vec<Id> = included_attr_defs()
                                                .iter()
                                                .map(|(id, _)| id.clone())
                                                .collect();
                                            handle_create_ent_def(
                                                    name(),
                                                    description.clone(),
                                                    listing_attr_def_id(),
                                                    attributes_ids,
                                                    saved,
                                                    err,
                                                )
                                                .await;
                                        }
                                    }
                                },
                                if saved() {
                                    "Close"
                                } else {
                                    "Create"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn handle_create_ent_def(
    name: String,
    description: Option<String>,
    listing_attr_def_id: Id,
    attr_def_ids: Vec<Id>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    let ent_def = EntityDef::new_with_attr_def_ids("".into(), name, description, attr_def_ids, listing_attr_def_id);
    log::debug!("Creating an entity definition {:?}: ", ent_def);
    match crate::server::fns::create_entity_def(ent_def).await {
        Ok(id) => {
            saved.set(true);
            err.set(None);
            Some(id)
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
            None
        }
    }
}
