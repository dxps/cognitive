use crate::{
    domain::model::{AttributeDef, Cardinality, EntityDef, Id},
    server::fns::{get_entity_link_def, remove_entity_def, update_entity_def},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, ConfirmationModal, Nav},
        pages::{meta::ent_def::fetch_all_attr_defs, EntityLinkDefForm, Name},
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Props, Clone)]
pub struct EntityDefPageProps {
    id: Id,
}

#[component]
pub fn EntityLinkDefPage(props: EntityDefPageProps) -> Element {
    //
    let id = use_signal(|| props.id);
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut cardinality = use_signal(|| Cardinality::OneToOne);

    let mut source_entity_def_id = use_signal(|| Id::default());
    let mut target_entity_def_id = use_signal(|| Id::default());
    let mut entity_defs = use_signal::<HashMap<Id, Name>>(|| HashMap::new());

    let mut included_attr_defs = use_signal(|| HashMap::<Id, String>::new());
    let mut all_attr_defs = use_signal(|| HashMap::<Id, String>::new());

    let mut show_modal = use_signal(|| false);
    let action_done = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let mut err: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
        entity_defs.set(UI_STATE.get_ent_defs().await);
    });

    use_future(move || async move {
        if let Some(item) = get_entity_link_def(id()).await.unwrap_or_default() {
            name.set(item.name);
            description.set(item.description.unwrap_or_default());
            cardinality.set(item.cardinality);
            source_entity_def_id.set(item.source_entity_def_id);
            target_entity_def_id.set(item.target_entity_def_id);
            let attrs = item.attributes.iter().map(|attr| (attr.id.clone(), attr.name.clone())).collect();
            included_attr_defs.set(attrs);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_ent_link_def(Route::EntityDefPage { id: id() }, name()) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(80px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity Link Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkDefListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-2" }
                        EntityLinkDefForm {
                            name,
                            description,
                            cardinality,
                            source_entity_def_id,
                            target_entity_def_id,
                            entity_defs,
                            included_attr_defs,
                            all_attr_defs,
                            action: action(),
                            action_done,
                            err
                        }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-300 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    show_modal.set(true);
                                },
                                "Delete"
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 min-w-[90px] disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    async move {
                                        match action().clone() {
                                            Action::View => {
                                                action.set(Action::Edit);
                                            }
                                            Action::Delete => {
                                                navigator().push(Route::EntityDefListPage {});
                                            }
                                            Action::Edit => {
                                                if action_done() {
                                                    navigator().push(Route::EntityDefListPage {});
                                                } else {
                                                    if name().is_empty() {
                                                        err.set(Some("Name cannot be empty".to_string()));
                                                        return;
                                                    }
                                                    let description = match description().is_empty() {
                                                        true => None,
                                                        false => Some(description()),
                                                    };
                                                    let attributes_ids: Vec<Id> = included_attr_defs()
                                                        .iter()
                                                        .map(|(id, _)| id.clone())
                                                        .collect();
                                                    handle_update(
                                                            id(),
                                                            name(),
                                                            description,
                                                            cardinality(),
                                                            source_entity_def_id(),
                                                            target_entity_def_id(),
                                                            attributes_ids,
                                                            all_attr_defs(),
                                                            included_attr_defs(),
                                                            action_done,
                                                            err,
                                                        )
                                                        .await;
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                },
                                if action() == Action::View || (action() == Action::Edit && action_done()) {
                                    "Edit"
                                } else if action() == Action::Delete {
                                    "Close"
                                } else {
                                    "Update"
                                }
                            }
                        }
                    }
                }
            }
            if show_modal() {
                if action() != Action::Delete {
                    ConfirmationModal {
                        title: "Confirm Delete",
                        content: "Are you sure you want to delete this entity link definition?",
                        action,
                        show_modal,
                        action_handler: move |_| {
                            spawn(async move {
                                log::debug!("[ent_link_def_page] Calling handle_delete ...");
                                handle_delete(&id(), action_done, err).await;
                            });
                        }
                    }
                }
            } else if action_done() {
                AcknowledgeModal {
                    title: "Confirmation",
                    content: if action() == Action::Delete {
                        "The entity definition has been successfully deleted."
                    } else {
                        "The entity definition has been successfully updated."
                    },
                    action_handler: move |_| {
                        navigator().push(Route::EntityDefListPage {});
                    }
                }
            } else if err().is_some() {
                AcknowledgeModal {
                    title: "Error",
                    content: if action() == Action::Delete {
                        format!(
                            "Failed to delete the entity link definition. Cause: '{}'.",
                            err().unwrap(),
                        )
                    } else {
                        format!(
                            "Failed to update the entity link definition. Cause: '{}'.",
                            err().unwrap(),
                        )
                    },
                    action_handler: move |_| {
                        err.set(None);
                    }
                }
            }
        }
    }
}

async fn handle_update(
    id: Id,
    name: String,
    description: Option<String>,
    cardinality: Cardinality,
    source_entity_def_id: Id,
    target_entity_def_id: Id,
    included_attr_def_ids: Vec<Id>,
    all_attr_defs: HashMap<Id, String>,
    included_attr_defs: HashMap<Id, String>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    //
    log::debug!(
        "[ent_link_def_page] Updating entity link definition with id:'{id}' name:{name} description:{:?} cardinality:{:?} source_entity_def_id:{:?} target_entity_def_id:{:?} included_attr_def_ids:{:?}: ",
        description,
        cardinality,
        source_entity_def_id,
        target_entity_def_id,
        included_attr_def_ids
    );

    let attributes: HashMap<Id, Name> = included_attr_def_ids
        .iter()
        .map(|id| {
            (
                id.clone(),
                all_attr_defs.get(id).unwrap_or(included_attr_defs.get(id).unwrap()).clone(),
            )
        })
        .collect();
    // let ent_def = EntityDef::new_with_attr_def_ids(id, name, description, attributes, listing_attr_def_id);
    // match update_entity_def(ent_def.clone()).await {
    //     Ok(_) => {
    //         saved.set(true);
    //         err.set(None);
    //         UI_STATE.update_ent_def(ent_def);
    //     }
    //     Err(e) => {
    //         saved.set(false);
    //         err.set(Some(e.to_string()));
    //     }
    // }
}

async fn handle_delete(id: &Id, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!("[ent_link_def_page] Deleting entity link definition: {:?}", id);
    // match remove_entity_def(id.clone()).await {
    //     Ok(_) => {
    //         action_done.set(true);
    //         err.set(None);
    //         UI_STATE.remove_ent_def(&id);
    //     }
    //     Err(e) => {
    //         action_done.set(false);
    //         err.set(Some(e.to_string()));
    //     }
    // }
}
