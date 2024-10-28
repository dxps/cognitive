use crate::{
    domain::model::{AttributeDef, Id},
    server::fns::{get_attribute_def, remove_attr_def, update_attribute_def},
    ui::{
        comps::{AcknowledgeModal, AttributeDefForm, Breadcrumb, ConfirmationModal, Nav},
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use std::sync::Arc;

#[derive(PartialEq, Props, Clone)]
pub struct AttributeDefEditPageProps {
    attr_def_id: Id,
}

#[component]
pub fn AttributeDefPage(props: AttributeDefEditPageProps) -> Element {
    //
    let id = use_signal(|| props.attr_def_id.clone());
    let mut attr_def = use_signal(|| None);
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut value_type = use_signal(|| "".to_string());
    let mut default_value = use_signal(|| "".to_string());
    let mut is_required = use_signal(|| false);
    let mut tag_id = use_signal(|| Id::default());
    let mut tags = use_signal(|| Arc::new(Vec::new()));

    let mut show_delete_confirm = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let action_done = use_signal(|| false);
    let mut err: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        tags.set(UI_STATE.get_tags_list().await);
    });

    use_future(move || async move {
        attr_def.set(get_attribute_def(id()).await.unwrap_or_default());
        if attr_def().is_some() {
            let item = attr_def().unwrap();
            name.set(item.name);
            description.set(item.description.unwrap_or_default());
            value_type.set(item.value_type.to_string());
            default_value.set(item.default_value);
            is_required.set(item.is_required);
            tag_id.set(item.tag_id.unwrap_or_default());
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb {
                paths: Route::get_path_to_attr_def(
                    Route::AttributeDefPage {
                        attr_def_id: id(),
                    },
                    name(),
                )
            }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Attribute Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-4" }
                        AttributeDefForm {
                            name,
                            description,
                            value_type,
                            default_value,
                            is_required,
                            tag_id,
                            tags: tags(),
                            action: action()
                        }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-200 bg-slate-50 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    show_delete_confirm.set(true);
                                },
                                "Delete"
                            }
                            button {
                                class: "bg-gray-100 enabled:hover:bg-green-100 disabled:text-gray-400 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    let curr_action = action().clone();
                                    async move {
                                        if curr_action == Action::View {
                                            action.set(Action::Edit);
                                        } else if action() == Action::Delete && action_done() {
                                            navigator().push(Route::AttributeDefListPage {});
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            let description = match description().is_empty() {
                                                true => None,
                                                false => Some(description()),
                                            };
                                            let tag_id = match tag_id().is_empty() {
                                                true => None,
                                                false => Some(tag_id()),
                                            };
                                            let item = AttributeDef::new(
                                                id(),
                                                name(),
                                                description,
                                                value_type().into(),
                                                default_value(),
                                                is_required(),
                                                tag_id,
                                            );
                                            handle_update(item, action_done, err).await;
                                            action.set(Action::View);
                                        }
                                    }
                                },
                                if action() == Action::View || (action() == Action::Delete && !action_done()) {
                                    "Edit"
                                } else if action() == Action::Delete && action_done() {
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
                ConfirmationModal {
                    title: "Confirm Delete",
                    content: "Are you sure you want to delete this attribute definition?",
                    action,
                    show_modal: show_delete_confirm,
                    action_handler: move |_| {
                        spawn(async move {
                            log::debug!("Calling handle_delete ...");
                            handle_delete(&id(), action_done, err).await;
                        });
                    }
                }
            } else if action_done() {
                AcknowledgeModal {
                    title: "Confirmation",
                    content: if action() == Action::Delete {
                        "The attribute definition has been successfully deleted."
                    } else {
                        "The attribute definition has been successfully updated."
                    },
                    action_handler: move |_| {
                        navigator().push(Route::AttributeDefListPage {});
                    }
                }
            }
        }
    }
}

async fn handle_update(item: AttributeDef, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Updating attribute definition: {:?}", item);
    match update_attribute_def(item).await {
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

async fn handle_delete(id: &Id, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Deleting attribute definition: {:?}", id);
    match remove_attr_def(id.clone()).await {
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
