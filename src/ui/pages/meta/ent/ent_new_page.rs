use std::collections::HashMap;

use crate::{
    domain::model::{
        AttributeValueType, BooleanAttribute, Entity, EntityDef, Id, IntegerAttribute, SmallintAttribute, TextAttribute,
    },
    ui::{
        comps::{Breadcrumb, EntityForm, Nav, Select},
        routes::Route,
        Action, UI_GLOBALS,
    },
};
use dioxus::prelude::*;

pub fn EntityNewPage() -> Element {
    //
    let mut ent_defs = use_signal::<HashMap<Id, EntityDef>>(|| HashMap::new());
    let mut ent_kinds = use_signal::<HashMap<Id, String>>(|| HashMap::new());
    let selected_kind_id = use_signal(|| "".to_string());

    // The attributes and the collected values (as `String`s).
    let mut text_attrs = use_signal::<HashMap<Id, (TextAttribute, String)>>(|| HashMap::new());
    let mut smallint_attrs = use_signal::<HashMap<Id, (SmallintAttribute, String)>>(|| HashMap::new());
    let mut int_attrs = use_signal::<HashMap<Id, (IntegerAttribute, String)>>(|| HashMap::new());
    let mut boolean_attrs = use_signal::<HashMap<Id, (BooleanAttribute, String)>>(|| HashMap::new());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        let defs = UI_GLOBALS.get_ent_defs().await;
        ent_kinds.set(defs.iter().map(|(id, def)| (id.clone(), def.name.clone())).collect());
        ent_defs.set(defs);
    });

    use_memo(move || {
        let kind_id = selected_kind_id();
        log::debug!("[EntityNewPage] Changed selected kind_id: {:?}", kind_id);
        if kind_id.len() > 0 {
            log::debug!("[EntityNewPage] Loading attributes from entity def id:'{}' ...", kind_id);
            let kind_id = kind_id.clone();
            log::debug!(
                "[EntityNewPage] Loading attributes from entity def id:'{}' using the global state ...",
                kind_id
            );
            if let Some(ent_def) = UI_GLOBALS.get_ent_def_sync(&kind_id) {
                let mut txt_attrs = HashMap::new();
                let mut si_attrs = HashMap::new();
                let mut i_attrs = HashMap::new();
                let mut b_attrs = HashMap::new();
                ent_def.attributes.iter().for_each(|attr| match &attr.value_type {
                    &AttributeValueType::Text => {
                        txt_attrs.insert(attr.id.clone(), (attr.clone().into(), "".to_string()));
                    }
                    &AttributeValueType::SmallInteger => {
                        si_attrs.insert(attr.id.clone(), (attr.into(), "".to_string()));
                    }
                    &AttributeValueType::Integer => {
                        i_attrs.insert(attr.id.clone(), (attr.into(), "".to_string()));
                    }
                    &AttributeValueType::Boolean => {
                        b_attrs.insert(attr.id.clone(), (attr.into(), "".to_string()));
                    }
                    _ => {}
                });
                text_attrs.set(txt_attrs);
                smallint_attrs.set(si_attrs);
                int_attrs.set(i_attrs);
                boolean_attrs.set(b_attrs);
                log::debug!("[EntityNewPage] Loaded attributes from entity def id:'{}'", kind_id);
            } else {
                log::warn!("[EntityNewPage] Failed to get entity def id:'{}'", kind_id);
            }
        };
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityListPage {},
                                "x"
                            }
                        }
                        hr { class: "flex" }
                        div { class: "flex py-4",
                            p { class: "py-2 pr-4 text-gray-600 block", "Kind:" }
                            if !ent_defs().is_empty() {
                                Select { items: ent_kinds, selected_item_id: selected_kind_id }
                            }
                        }
                        if selected_kind_id().len() == 0 {
                            p { class: "py-2 text-gray-500 block",
                                "Select a kind to create an entity."
                            }
                        } else {
                            EntityForm {
                                text_attrs,
                                smallint_attrs,
                                int_attrs,
                                boolean_attrs,
                                action: Action::Edit,
                                saved,
                                err
                            }
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
                            button { class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                // async move {
                                //     if saved() {
                                //         navigator().push(Route::EntityDefListPage {});
                                //     } else {
                                //         let attributes_ids: Vec<Id> = included_attr_defs()
                                //             .iter()
                                //             .map(|(id, _)| id.clone())
                                //             .collect();
                                //         handle_create_ent_def(
                                //                 name(),
                                //                 description.clone(),
                                //                 attributes_ids,
                                //                 saved,
                                //                 err,
                                //             )
                                //             .await;
                                //     }
                                // }
                                // },
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

async fn handle_create_ent(
    kind: String,
    text_attrs: Vec<TextAttribute>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    //

    let ent = Entity::new_from(kind, text_attrs);

    log::debug!("Creating the entity {:?} ...", ent);

    match crate::server::fns::create_entity(ent).await {
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
