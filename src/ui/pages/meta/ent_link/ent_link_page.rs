use crate::{
    domain::model::{BooleanAttribute, Id, IntegerAttribute, SmallintAttribute, TextAttribute},
    server::fns::{get_entity_link, remove_entity_link},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, ConfirmationModal, EntityLinkForm, Nav},
        pages::Name,
        routes::Route,
        Action,
    },
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Props, Clone)]
pub struct EntityLinkPageProps {
    id: Id,
}

#[component]
pub fn EntityLinkPage(props: EntityLinkPageProps) -> Element {
    //
    let id = use_signal(|| props.id);

    let kind = use_signal(|| Name::default());

    let mut def_source_entity_id = use_signal(|| Id::default());
    let mut def_target_entity_id = use_signal(|| Id::default());

    let source_entity_id = use_signal(|| Id::default());
    let target_entity_id = use_signal(|| Id::default());

    let mut source_entities_id_name = use_signal(|| HashMap::<Id, Name>::new());
    let mut target_entities_id_name = use_signal(|| HashMap::<Id, Name>::new());

    let mut text_attrs = use_signal::<HashMap<Id, TextAttribute>>(|| HashMap::new());
    let mut smallint_attrs = use_signal::<HashMap<Id, SmallintAttribute>>(|| HashMap::new());
    let mut int_attrs = use_signal::<HashMap<Id, IntegerAttribute>>(|| HashMap::new());
    let mut boolean_attrs = use_signal::<HashMap<Id, BooleanAttribute>>(|| HashMap::new());

    let mut show_delete_confirm = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let action_done = use_signal(|| false);
    let err: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        init(id, kind, text_attrs, smallint_attrs, int_attrs, boolean_attrs).await;
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_ent_link(id(), format!("{} ({})", kind(), id())) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity Link"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-2" }
                        EntityLinkForm {
                            source_entity_id,
                            source_entities_id_name,
                            target_entity_id,
                            target_entities_id_name,
                            text_attrs,
                            smallint_attrs,
                            int_attrs,
                            boolean_attrs,
                            action: Action::Edit
                        }
                        hr { class: "mt-8 mb-1" }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-300 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    show_delete_confirm.set(true);
                                },
                                "Delete"
                            }
                            // Show the buttons's action result in the UI.
                            div { class: "min-w-[400px] max-w-[400px] text-sm flex justify-center items-center",
                                if err().is_some() {
                                    span { class: "text-red-600", { err().unwrap() } }
                                } else if action_done() {
                                    span { class: "text-green-600",
                                        {
                                            if action() == Action::Edit {
                                                "Successfully updated"
                                            } else {
                                                ""
                                            }
                                        }
                                    }
                                }
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
                                                navigator().push(Route::EntityListPage {});
                                            }
                                            Action::Edit => {
                                                if action_done() {
                                                    navigator().push(Route::EntityListPage {});
                                                } else {
                                                    handle_update(
                                                            id(),
                                                            source_entity_id(),
                                                            target_entity_id(),
                                                            text_attrs().values().cloned().collect(),
                                                            smallint_attrs().values().cloned().collect(),
                                                            int_attrs().values().cloned().collect(),
                                                            boolean_attrs().values().cloned().collect(),
                                                            action_done,
                                                            err,
                                                        )
                                                        .await;
                                                    action.set(Action::View);
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
            if show_delete_confirm() {
                if action() != Action::Delete {
                    ConfirmationModal {
                        title: "Confirm Delete",
                        content: "Are you sure you want to delete this entity link?",
                        action,
                        show_modal: show_delete_confirm,
                        action_handler: move |_| {
                            spawn(async move {
                                log::debug!("Calling handle_delete ...");
                                handle_delete(&id(), action_done, err).await;
                            });
                        }
                    }
                }
            } else if action_done() {
                AcknowledgeModal {
                    title: "Confirmation",
                    content: if action() == Action::Delete {
                        vec!["The entity link has been successfully deleted.".into()]
                    } else {
                        vec!["The entity link has been successfully updated.".into()]
                    },
                    action_handler: move |_| {
                        navigator().push(Route::EntityLinkListPage {});
                    }
                }
            }
        }
    }
}

async fn init(
    id: Signal<Id>,
    mut kind: Signal<Name>,
    mut text_attrs: Signal<HashMap<Id, TextAttribute>>,
    mut smallint_attrs: Signal<HashMap<Id, SmallintAttribute>>,
    mut int_attrs: Signal<HashMap<Id, IntegerAttribute>>,
    mut boolean_attrs: Signal<HashMap<Id, BooleanAttribute>>,
) {
    match get_entity_link(id()).await {
        Ok(Some(ent)) => {
            log::debug!("[EntityLinkPage] Based on id {id}, got entity {:?}", ent);
            let attrs: HashMap<Id, TextAttribute> = ent
                .text_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            text_attrs.set(attrs);
            let attrs: HashMap<Id, SmallintAttribute> = ent
                .smallint_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            smallint_attrs.set(attrs);
            let attrs: HashMap<Id, IntegerAttribute> = ent
                .int_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            int_attrs.set(attrs);
            let attrs: HashMap<Id, BooleanAttribute> = ent
                .boolean_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            boolean_attrs.set(attrs);
            kind.set(ent.kind);
        }
        Ok(None) => {
            log::error!("[EntityLinkPage] Entity link with id '{id}' not found.");
        }
        Err(err) => {
            log::error!("[EntityLinkPage] Failed to get entity by id '{id}'. Cause: {err}");
        }
    }
}

async fn handle_update(
    ent_link_id: Id,
    source_entity_id: Id,
    target_entity_id: Id,
    text_attributes: Vec<TextAttribute>,
    smallint_attributes: Vec<SmallintAttribute>,
    int_attributes: Vec<IntegerAttribute>,
    boolean_attributes: Vec<BooleanAttribute>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    //
    // let ent = Entity::new_with_id_attrs(
    //     ent_id,
    //     kind,
    //     def_id,
    //     text_attributes,
    //     smallint_attributes,
    //     int_attributes,
    //     boolean_attributes,
    //     listing_attr_def_id,
    // );

    // log::debug!("Updating entity '{:?}' ... ", ent);

    // match update_entity(ent).await {
    //     Ok(_) => {
    //         saved.set(true);
    //         err.set(None);
    //     }
    //     Err(e) => {
    //         saved.set(false);
    //         err.set(Some(e.to_string()));
    //     }
    // }
}

async fn handle_delete(id: &Id, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!("[EntityLinkPage] Deleting entity link w/ id {:?}", id);
    match remove_entity_link(id.clone()).await {
        Ok(_) => {
            action_done.set(true);
            err.set(None);
        }
        Err(e) => {
            action_done.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
