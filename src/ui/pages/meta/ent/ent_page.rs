use crate::{
    domain::model::{BooleanAttribute, Id, IntegerAttribute, SmallintAttribute, TextAttribute},
    server::fns::get_entity,
    ui::{
        comps::{Breadcrumb, EntityForm, Nav},
        routes::Route,
        Action,
    },
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Props, Clone)]
pub struct EntityPageProps {
    id: Id,
}

#[component]
pub fn EntityPage(props: EntityPageProps) -> Element {
    //
    let id = use_signal(|| props.id);

    let kind = use_signal(|| "".to_string());

    let text_attrs = use_signal::<HashMap<Id, TextAttribute>>(|| HashMap::new());
    let smallint_attrs = use_signal::<HashMap<Id, SmallintAttribute>>(|| HashMap::new());
    let int_attrs = use_signal::<HashMap<Id, IntegerAttribute>>(|| HashMap::new());
    let boolean_attrs = use_signal::<HashMap<Id, BooleanAttribute>>(|| HashMap::new());

    let mut action = use_signal(|| Action::View);
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        init(id, kind, text_attrs).await;
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb {
                paths: Route::get_path_to_ent(
                    Route::EntityPage { id: id() },
                    format!("{} ({})", kind(), id()),
                )
            }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-2" }
                        EntityForm {
                            text_attrs,
                            smallint_attrs,
                            int_attrs,
                            boolean_attrs,
                            action: action(),
                            err
                        }
                        hr { class: "mt-8 mb-1" }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-300 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    action.set(Action::Delete);
                                    async move { handle_delete(id(), saved, err).await }
                                },
                                "Delete"
                            }
                            // Show the buttons's action result in the UI.
                            div { class: "min-w-[400px] max-w-[400px] text-sm flex justify-center items-center",
                                if err().is_some() {
                                    span { class: "text-red-600", { err().unwrap() } }
                                } else if saved() {
                                    span { class: "text-green-600",
                                        {
                                            if action() == Action::Edit {
                                                "Successfully updated"
                                            } else if action() == Action::Delete {
                                                "Successfully deleted"
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
                                                navigator().push(Route::EntityDefListPage {});
                                            }
                                            Action::Edit => {
                                                if saved() {
                                                    navigator().push(Route::EntityDefListPage {});
                                                } else {
                                                    handle_update(id(), saved, err).await;
                                                }
                                            }
                                        }
                                    }
                                },
                                if action() == Action::View || (action() == Action::Edit && saved()) {
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
        }
    }
}

async fn init(id: Signal<Id>, mut kind: Signal<String>, mut text_attrs: Signal<HashMap<Id, TextAttribute>>) {
    match get_entity(id()).await {
        Ok(Some(ent)) => {
            log::debug!("[EntityPage] Based on id {id}, got entity {:?}", ent);
            let ta: HashMap<Id, TextAttribute> = ent
                .text_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            text_attrs.set(ta);
            kind.set(ent.kind);
        }
        Ok(None) => {
            log::error!("[EntityPage] Entity with id '{id}' not found");
        }
        Err(err) => {
            log::error!("[EntityPage] Failed to get entity by id '{id}'. Cause: {err}");
        }
    }
}

async fn handle_update(id: Id, mut _saved: Signal<bool>, mut _err: Signal<Option<String>>) {
    //
    log::debug!("Updating entity w/ id:'{id}' ... ");

    // let item = EntityDef::new_with_attr_def_ids(id, name, description, attr_def_ids);
    // match update_entity_def(item).await {
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

async fn handle_delete(id: Id, mut _saved: Signal<bool>, mut _err: Signal<Option<String>>) {
    //
    log::debug!(">>> Deleting entity w/ id {:?}", id);
    // match remove_entity_def(id.clone()).await {
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
