use crate::{
    domain::model::{EntityDef, Id, ItemType},
    server::{
        fns::{get_entity_def, list_entities_refs_by_def_id, remove_entity_def, update_entity_def},
        AppError,
    },
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, ConfirmationModal, Nav},
        pages::{meta::ent_def::fetch_all_attr_defs, EntityDefForm, Name},
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
pub fn EntityDefPage(props: EntityDefPageProps) -> Element {
    //
    let id = use_signal(|| props.id);
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut included_attr_defs = use_signal(|| HashMap::<Id, String>::new());
    let mut listing_attr_def_id = use_signal(|| Id::default());

    let mut all_attr_defs = use_signal(|| HashMap::<Id, String>::new());

    let mut show_modal = use_signal(|| false);
    let action_done = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let err_refs: Signal<Vec<(Id, Name)>> = use_signal(|| Vec::new());

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
    });

    use_future(move || async move {
        if let Some(item) = get_entity_def(id()).await.unwrap_or_default() {
            name.set(item.name);
            description.set(item.description.unwrap_or_default());
            let attrs = item.attributes.iter().map(|attr| (attr.id.clone(), attr.name.clone())).collect();
            included_attr_defs.set(attrs);
            // Remove the items that exist in `included_attr_defs` from `all_attr_defs`.
            let included_ids = included_attr_defs().iter().map(|item| item.0.clone()).collect::<Vec<Id>>();
            let mut temp = all_attr_defs();
            temp.retain(|id, _| !included_ids.contains(&id));
            all_attr_defs.set(temp);
            listing_attr_def_id.set(item.listing_attr_def_id);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_ent_def(Route::EntityDefPage { id: id() }, name()) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(80px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-10",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityDefListPage {},
                                "X"
                            }
                        }
                        EntityDefForm {
                            name,
                            description,
                            included_attr_defs,
                            listing_attr_def_id,
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
                                                    if included_attr_defs().is_empty() {
                                                        err.set(Some("Include at least one attribute".to_string()));
                                                        return;
                                                    }
                                                    let attributes_ids: Vec<Id> = included_attr_defs()
                                                        .iter()
                                                        .map(|(id, _)| id.clone())
                                                        .collect();
                                                    handle_update(
                                                            id(),
                                                            name(),
                                                            description,
                                                            attributes_ids,
                                                            listing_attr_def_id(),
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
                        content: "Are you sure you want to delete this entity definition?",
                        action,
                        show_modal,
                        action_handler: move |_| {
                            spawn(async move {
                                log::debug!("[ent_def_page] Calling handle_delete ...");
                                handle_delete(&id(), action_done, err, err_refs).await;
                            });
                        }
                    }
                }
            } else if action_done() {
                AcknowledgeModal {
                    title: "Confirmation",
                    content: if action() == Action::Delete {
                        vec!["The entity definition has been successfully deleted.".into()]
                    } else {
                        vec!["The entity definition has been successfully updated.".into()]
                    },
                    action_handler: move |_| {
                        navigator().push(Route::EntityDefListPage {});
                    }
                }
            } else if err().is_some() {
                AcknowledgeModal {
                    title: "Error",
                    content: vec![err().unwrap()],
                    links_item_type: ItemType::Entity,
                    links: err_refs(),
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
    included_attr_def_ids: Vec<Id>,
    listing_attr_def_id: Id,
    all_attr_defs: HashMap<Id, String>,
    included_attr_defs: HashMap<Id, String>,
    mut action_done: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    //
    log::debug!(
        "[ent_def_page] Updating entity definition with id:'{id}' name:{name} description:{:?} included_attr_def_ids:{:?}: ",
        description,
        included_attr_def_ids
    );
    let attributes = included_attr_def_ids
        .iter()
        .map(|id| {
            (
                id.clone(),
                all_attr_defs.get(id).unwrap_or(included_attr_defs.get(id).unwrap()).clone(),
            )
        })
        .collect();
    let ent_def = EntityDef::new_with_attr_def_ids(id, name, description, attributes, listing_attr_def_id);
    match update_entity_def(ent_def.clone()).await {
        Ok(_) => {
            action_done.set(true);
            err.set(None);
            UI_STATE.update_ent_def(ent_def);
        }
        Err(e) => {
            action_done.set(false);
            if let ServerFnError::ServerError(s) = e {
                err.set(Some(s));
            } else {
                err.set(Some(e.to_string()));
            }
        }
    }
}

async fn handle_delete(id: &Id, mut action_done: Signal<bool>, mut err: Signal<Option<String>>, mut err_refs: Signal<Vec<(Id, String)>>) {
    //
    log::debug!("[ent_def_page] Deleting entity definition: {:?}", id);
    match remove_entity_def(id.clone()).await {
        Ok(_) => {
            action_done.set(true);
            err.set(None);
            UI_STATE.remove_ent_def(&id);
        }
        Err(e) => {
            action_done.set(false);
            if let ServerFnError::ServerError(s) = e {
                log::debug!(
                    ">>> s={:?} AppError::DependenciesExist.to_string()={:?}.",
                    s,
                    AppError::DependenciesExist.to_string()
                );
                if s == AppError::DependenciesExist.to_string() {
                    if let Ok(refs) = list_entities_refs_by_def_id(id.clone()).await {
                        err.set(Some("Cannot delete it because it is refered by the following entities:".into()));
                        err_refs.set(refs);
                    } else {
                        err.set(Some("Cannot delete it because it is refered by one or more entities.".into()));
                        log::error!(">>> Failed to delete entity definition, but no entities referring to it were found.");
                    }
                }
            } else {
                err.set(Some(e.to_string()));
            }
        }
    }
}
