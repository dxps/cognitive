use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
    domain::model::{EntityDef, Id},
    server::fns::{get_entity_def, remove_entity_def, update_entity_def},
    ui::{
        comps::{Breadcrumb, Nav},
        pages::{meta::ent_def::fetch_all_attr_defs, EntityDefForm},
        routes::Route,
        Action,
    },
};

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
    let mut included_attr_defs = use_signal(|| Vec::<(Id, String)>::new());
    let mut all_attr_defs = use_signal(|| HashMap::<Id, String>::new());

    let mut action = use_signal(|| Action::View);
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
    });

    use_future(move || async move {
        if let Some(item) = get_entity_def(id()).await.unwrap_or_default() {
            name.set(item.name);
            description.set(item.description.unwrap_or_default());
            let attrs = item
                .attributes
                .iter()
                .map(|attr| (attr.id.clone(), attr.name.clone()))
                .collect();
            included_attr_defs.set(attrs);
            // Remove the items that exist in `included_attr_defs` from `all_attr_defs`.
            let included_ids = included_attr_defs().iter().map(|item| item.0.clone()).collect::<Vec<Id>>();
            let mut temp = all_attr_defs();
            temp.retain(|id, _| !included_ids.contains(&id));
            all_attr_defs.set(temp);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_ent_def(Route::EntityDefPage { id: id() }, name()) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                        EntityDefForm {
                            name,
                            description,
                            included_attr_defs,
                            all_attr_defs,
                            action: action(),
                            err
                        }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-200 bg-slate-50 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    action.set(Action::Delete);
                                    async move { handle_delete(id(), saved, err).await }
                                },
                                "Delete"
                            }
                            // Show the buttons's action result in the UI.
                            div { class: "min-w-[440px] max-w-[440px]",
                                if err().is_some() {
                                    span { class: "text-red-600 flex justify-center",
                                        { err().unwrap() }
                                    }
                                } else if saved() {
                                    span { class: "text-green-600 flex justify-center",
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
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: included_attr_defs().is_empty(),
                                onclick: move |_| {
                                    async move {
                                        if action().clone() == Action::View {
                                            action.set(Action::Edit);
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
                                            handle_update(id(), name(), description, attributes_ids, saved, err)
                                                .await;
                                            included_attr_defs.set(vec![]);
                                        }
                                    }
                                },
                                if action() == Action::View {
                                    "Edit"
                                } else if action() == Action::Delete {
                                    "  -  "
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

async fn handle_update(
    id: Id,
    name: String,
    description: Option<String>,
    attr_def_ids: Vec<Id>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    //
    log::debug!(
        "Updating entity definition with id:'{id}' name:{name} description:{:?} attr_def_ids:{:?}: ",
        description,
        attr_def_ids
    );

    let item = EntityDef::new_with_attr_def_ids(id, name, description, attr_def_ids);
    match update_entity_def(item).await {
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

async fn handle_delete(id: Id, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Deleting entity definition: {:?}", id);
    match remove_entity_def(id.clone()).await {
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
