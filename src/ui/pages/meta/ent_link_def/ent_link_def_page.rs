use crate::{
    domain::model::{AttributeDef, Cardinality, EntityLinkDef, Id},
    server::fns::{get_entity_link_def, remove_entity_link_def, update_entity_link_def},
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
pub struct EntityLinkDefPageProps {
    id: Id,
}

#[component]
pub fn EntityLinkDefPage(props: EntityLinkDefPageProps) -> Element {
    //
    let id = use_signal(|| props.id);
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut cardinality_id = use_signal(|| Id::from(Cardinality::OneToOne.as_string()));

    let mut source_ent_def_id = use_signal(|| Id::default());
    let mut target_ent_def_id = use_signal(|| Id::default());
    let mut ent_defs = use_signal::<HashMap<Id, Name>>(|| HashMap::new());

    let mut included_attr_defs = use_signal(|| HashMap::<Id, Name>::new());
    let mut all_attr_defs = use_signal(|| HashMap::<Id, Name>::new());

    let mut show_modal = use_signal(|| false);
    let action_done = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let mut err: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
        ent_defs.set(UI_STATE.get_ent_defs().await);
    });

    use_future(move || async move {
        if let Some(item) = get_entity_link_def(id()).await.unwrap_or_default() {
            name.set(item.name);
            description.set(item.description.unwrap_or_default());
            cardinality_id.set(Id::from(item.cardinality.as_string()));
            source_ent_def_id.set(item.source_entity_def_id);
            target_ent_def_id.set(item.target_entity_def_id);
            if item.attributes.is_some() {
                let attrs = item
                    .attributes
                    .unwrap()
                    .iter()
                    .map(|attr| (attr.id.clone(), attr.name.clone()))
                    .collect();
                included_attr_defs.set(attrs);
            }
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_ent_link_def(id(), name()) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(80px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-12",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity Link Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkDefListPage {},
                                "X"
                            }
                        }
                        EntityLinkDefForm {
                            name,
                            description,
                            cardinality_id,
                            source_ent_def_id,
                            target_ent_def_id,
                            ent_defs,
                            included_attr_defs,
                            all_attr_defs,
                            action: action(),
                            action_done,
                            err
                        }
                        div { class: "flex justify-between mt-12",
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
                                                            cardinality_id(),
                                                            source_ent_def_id(),
                                                            target_ent_def_id(),
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
                        vec!["The entity link definition has been successfully deleted.".into()]
                    } else {
                        vec!["The entity link definition has been successfully updated.".into()]
                    },
                    action_handler: move |_| {
                        navigator().push(Route::EntityLinkDefListPage {});
                    }
                }
            } else if err().is_some() {
                AcknowledgeModal {
                    title: "Error",
                    content: if action() == Action::Delete {
                        vec![
                            "Failed to delete the entity link definition. Reason:".into(),
                            err().unwrap(),
                        ]
                    } else {
                        vec![
                            "Failed to update the entity link definition. Reason:".into(),
                            err().unwrap(),
                        ]
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
    cardinality_id: Id,
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
        "[ent_link_def_page] Updating entity link definition with id:'{id}' name:{name} description:{:?} cardinality_id:{:?} source_entity_def_id:{:?} target_entity_def_id:{:?} included_attr_def_ids:{:?}: ",
        description,
        cardinality_id,
        source_entity_def_id,
        target_entity_def_id,
        included_attr_def_ids
    );

    let attributes: Vec<AttributeDef> = included_attr_def_ids
        .iter()
        .map(|id| {
            let name = all_attr_defs.get(id).unwrap_or(included_attr_defs.get(id).unwrap()).clone();
            AttributeDef::new_with_id_name(id.clone(), name)
        })
        .collect();
    let attributes = if attributes.len() > 0 { Some(attributes) } else { None };
    let ent_link_def = EntityLinkDef::new(
        id,
        name,
        description,
        Cardinality::from(cardinality_id.as_str()),
        source_entity_def_id,
        target_entity_def_id,
        attributes,
    );
    match update_entity_link_def(ent_link_def.clone()).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
    UI_STATE.update_ent_link_def(ent_link_def);
}

async fn handle_delete(id: &Id, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!("[ent_link_def_page] Deleting entity link definition: {:?}", id);
    match remove_entity_link_def(id.clone()).await {
        Ok(_) => {
            action_done.set(true);
            err.set(None);
            UI_STATE.remove_ent_link_def(&id);
        }
        Err(e) => {
            action_done.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
