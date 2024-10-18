use crate::{
    domain::model::{
        AttributeValueType, BooleanAttribute, Entity, EntityDef, Id, IntegerAttribute, SmallintAttribute, TextAttribute,
    },
    ui::{
        comps::{Breadcrumb, EntityForm, Nav, Select},
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use std::collections::HashMap;

pub fn EntityNewPage() -> Element {
    //
    let mut ent_defs = use_signal::<HashMap<Id, EntityDef>>(|| HashMap::new());
    let mut ent_kinds = use_signal::<HashMap<Id, String>>(|| HashMap::new());
    let selected_kind_id = use_signal(|| Id::default());
    let mut selected_kind_name = use_signal(|| "".into());
    let mut listing_attr_def_id = use_signal(|| Id::default());
    let mut listing_attr_name = use_signal(|| "".to_string());
    let listing_attr_value = use_signal(|| "".to_string());

    let mut text_attrs = use_signal::<HashMap<Id, TextAttribute>>(|| HashMap::new());
    let mut smallint_attrs = use_signal::<HashMap<Id, SmallintAttribute>>(|| HashMap::new());
    let mut int_attrs = use_signal::<HashMap<Id, IntegerAttribute>>(|| HashMap::new());
    let mut boolean_attrs = use_signal::<HashMap<Id, BooleanAttribute>>(|| HashMap::new());

    let err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        let defs = UI_STATE.get_ent_defs().await;
        ent_kinds.set(defs.iter().map(|(id, def)| (id.clone(), def.name.clone())).collect());
        ent_defs.set(defs);
    });

    use_memo(move || {
        let kind_id = selected_kind_id();
        log::debug!("[EntityNewPage] Changed selected kind_id: {:?}", kind_id);
        if kind_id.is_empty() {
            return;
        }
        selected_kind_name.set(ent_kinds().get(&kind_id).unwrap().clone());
        log::debug!("[EntityNewPage] Loading attributes from entity def id:'{}' ...", kind_id);
        // let kind_id = kind_id.clone();
        log::debug!(
            "[EntityNewPage] Loading attributes from entity def id:'{}' using the global state ...",
            kind_id
        );
        if let Some(ent_def) = UI_STATE.get_ent_def_sync(&kind_id) {
            let mut txt_attrs = HashMap::new();
            let mut si_attrs = HashMap::new();
            let mut i_attrs = HashMap::new();
            let mut b_attrs = HashMap::new();
            ent_def.attributes.iter().for_each(|attr_def| {
                if attr_def.id == ent_def.listing_attr_def_id {
                    listing_attr_def_id.set(attr_def.id.clone());
                    listing_attr_name.set(attr_def.name.clone());
                }
                match &attr_def.value_type {
                    &AttributeValueType::Text => {
                        txt_attrs.insert(attr_def.id.clone(), attr_def.clone().into());
                    }
                    &AttributeValueType::SmallInteger => {
                        si_attrs.insert(attr_def.id.clone(), attr_def.into());
                    }
                    &AttributeValueType::Integer => {
                        i_attrs.insert(attr_def.id.clone(), attr_def.into());
                    }
                    &AttributeValueType::Boolean => {
                        b_attrs.insert(attr_def.id.clone(), attr_def.into());
                    }
                    _ => {}
                }
            });
            text_attrs.set(txt_attrs);
            smallint_attrs.set(si_attrs);
            int_attrs.set(i_attrs);
            boolean_attrs.set(b_attrs);
            log::debug!("[EntityNewPage] Loaded attributes from entity def id:'{}'", kind_id);
        } else {
            log::warn!("[EntityNewPage] Failed to get entity def id:'{}'", kind_id);
        }
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
                        if selected_kind_id().is_empty() {
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
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    async move {
                                        if saved() {
                                            navigator().push(Route::EntityListPage {});
                                        } else {
                                            handle_create_ent(
                                                    selected_kind_name(),
                                                    selected_kind_id(),
                                                    text_attrs().values().cloned().collect(),
                                                    smallint_attrs().values().cloned().collect(),
                                                    int_attrs().values().cloned().collect(),
                                                    boolean_attrs().values().cloned().collect(),
                                                    listing_attr_def_id(),
                                                    listing_attr_name(),
                                                    listing_attr_value(),
                                                    saved,
                                                    err,
                                                )
                                                .await;
                                        }
                                    }
                                },
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
    kind: String, // TODO: Review the usage of kind.
    def_id: Id,
    text_attrs: Vec<TextAttribute>,
    smallint_attrs: Vec<SmallintAttribute>,
    int_attrs: Vec<IntegerAttribute>,
    boolean_attrs: Vec<BooleanAttribute>,
    listing_attr_def_id: Id,
    listing_attr_name: String,
    listing_attr_value: String,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    //

    let ent = Entity::new(
        kind,
        def_id,
        text_attrs,
        smallint_attrs,
        int_attrs,
        boolean_attrs,
        listing_attr_def_id,
        listing_attr_name,
        listing_attr_value,
    );

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
