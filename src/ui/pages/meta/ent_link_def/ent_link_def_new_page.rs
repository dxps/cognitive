use crate::{
    domain::model::{AttributeDef, Cardinality, EntityLinkDef, Id},
    server::fns::create_entity_link_def,
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, Nav},
        pages::{fetch_all_attr_defs, EntityLinkDefForm, Name},
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn EntityLinkDefNewPage() -> Element {
    //
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let mut source_ent_def_id = use_signal(|| Id::default());
    let mut target_ent_def_id = use_signal(|| Id::default());
    let mut ent_defs = use_signal::<HashMap<Id, Name>>(|| HashMap::new());

    let mut cardinality_id = use_signal(|| Id::from(Cardinality::OneToOne.as_string()));

    let mut included_attr_defs = use_signal(|| HashMap::<Id, Name>::new());
    let mut all_attr_defs = use_signal(|| HashMap::<Id, Name>::new());

    let mut show_modal = use_signal(|| false);
    let action_done = use_signal(|| false);
    let mut action = use_signal(|| Action::Create);
    let mut err: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
        ent_defs.set(UI_STATE.get_ent_defs().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityLinkDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-12",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity Link Definition"
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
                        div { class: "flex justify-end mt-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: !form_is_valid(name, source_ent_def_id, target_ent_def_id),
                                onclick: move |_| {
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    async move {
                                        if action_done() {
                                            navigator().push(Route::EntityLinkDefListPage {});
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            handle_create_ent_link_def(
                                                    name(),
                                                    description,
                                                    cardinality_id(),
                                                    source_ent_def_id(),
                                                    target_ent_def_id(),
                                                    included_attr_defs(),
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
                AcknowledgeModal {
                    title: "Confirmation",
                    content: "The entity link definition has been successfully created.",
                    action_handler: move |_| {
                        navigator().push(Route::EntityLinkDefListPage {});
                    }
                }
            }
        }
    }
}

fn form_is_valid(name: Signal<String>, source_ent_def_id: Signal<Id>, target_ent_def_id: Signal<Id>) -> bool {
    // TODO
    true
}

async fn handle_create_ent_link_def(
    name: String,
    description: Option<String>,
    cardinality_id: Id,
    source_entity_def_id: Id,
    target_entity_def_id: Id,
    included_attr_defs: HashMap<Id, Name>,
    mut action_done: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    log::debug!("[handle_create_ent_link_def] Creating ent link def w/ name: '{:?}' ...", name);

    let attrs: Vec<AttributeDef> = included_attr_defs
        .iter()
        .map(|(id, name)| AttributeDef::new_with_id_name(id.clone(), name.clone()))
        .collect();
    let attrs = if attrs.len() > 0 { Some(attrs) } else { None };
    let mut ent_link_def = EntityLinkDef::from(
        name,
        description,
        Cardinality::from(cardinality_id.as_str()),
        source_entity_def_id,
        target_entity_def_id,
        attrs,
    );
    match create_entity_link_def(ent_link_def.clone()).await {
        Ok(id) => {
            action_done.set(true);
            err.set(None);
            ent_link_def.id = id;
            UI_STATE.add_ent_link_def(ent_link_def);
        }
        Err(e) => {
            action_done.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
