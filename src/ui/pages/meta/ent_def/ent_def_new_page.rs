use crate::{
    domain::model::{EntityDef, Id},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, Nav},
        pages::{meta::ent_def::fetch_all_attr_defs, EntityDefForm},
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use std::collections::HashMap;

pub fn EntityDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let included_attr_defs = use_signal::<HashMap<Id, String>>(|| HashMap::new());
    let listing_attr_def_id = use_signal(|| Id::default());

    let mut all_attr_defs = use_signal(|| HashMap::<Id, String>::new());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let action_done = use_signal(|| false);

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
                                "X"
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
                            action_done,
                            err
                        }
                        div { class: "flex justify-end mt-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: included_attr_defs().is_empty(),
                                onclick: move |_| {
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    async move {
                                        if action_done() {
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
                                            handle_create_ent_def(
                                                    name(),
                                                    description.clone(),
                                                    listing_attr_def_id(),
                                                    included_attr_defs(),
                                                    all_attr_defs(),
                                                    action_done,
                                                    err,
                                                )
                                                .await;
                                        }
                                    }
                                },
                                if action_done() {
                                    "Close"
                                } else {
                                    "Create"
                                }
                            }
                        }
                    }
                }
            }
            if action_done() {
                if err().is_none() {
                    AcknowledgeModal {
                        title: "Confirmation",
                        content: "The entity definition has been successfully created.",
                        action_handler: move |_| {
                            navigator().push(Route::EntityDefListPage {});
                        }
                    }
                } else {
                    AcknowledgeModal {
                        title: "Error",
                        content: "Failed to create the entity definition. Cause: '{err.unwrap()}' deleted.",
                        action_handler: move |_| {
                            navigator().push(Route::EntityListPage {});
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
    included_attr_defs: HashMap<Id, String>,
    all_attr_defs: HashMap<Id, String>,
    mut action_done: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    log::debug!(
        "[handle_create_ent_def] Creating ent def w/ included_attr_defs: {:?} and all_attr_defs: {:?} ",
        included_attr_defs,
        all_attr_defs
    );

    let mut ent_def = EntityDef::new_with_attr_def_ids("".into(), name, description, included_attr_defs, listing_attr_def_id);
    log::debug!("[handle_create_ent_def] Creating ent def: {:?}: ", ent_def);
    match crate::server::fns::create_entity_def(ent_def.clone()).await {
        Ok(id) => {
            action_done.set(true);
            err.set(None);
            ent_def.id = id;
            UI_STATE.add_ent_def(ent_def);
        }
        Err(e) => {
            action_done.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
