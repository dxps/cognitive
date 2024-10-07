use std::collections::HashMap;

use crate::{
    domain::model::{BooleanAttribute, Entity, EntityDef, Id, SmallintAttribute, TextAttribute},
    ui::{
        comps::{Breadcrumb, Nav, Select},
        pages::EntityForm,
        routes::Route,
        Action, UI_GLOBALS,
    },
};
use dioxus::prelude::*;

pub fn EntityNewPage() -> Element {
    //
    let mut ent_kinds = use_signal::<HashMap<Id, String>>(|| HashMap::new());
    let mut selected_kind_id = use_signal(|| "".to_string());
    let ent_attrs = use_signal::<Vec<Entity>>(|| Vec::new());

    let text_attrs = use_signal::<HashMap<Id, (TextAttribute, String)>>(|| HashMap::new());
    let smallint_attrs = use_signal::<HashMap<Id, (SmallintAttribute, String)>>(|| HashMap::new());
    let boolean_attrs = use_signal::<HashMap<Id, (BooleanAttribute, String)>>(|| HashMap::new());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        ent_kinds.set(UI_GLOBALS.get_ent_kinds().await);
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
                        Select { items: ent_kinds, selected_item_id: selected_kind_id }
                        EntityForm {
                            kind: selected_kind_id,
                            text_attrs,
                            smallint_attrs,
                            boolean_attrs,
                            action: Action::Edit,
                            saved,
                            err
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

async fn handle_create_ent_def(
    name: String,
    description: Option<String>,
    attr_def_ids: Vec<Id>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    let ent_def = EntityDef::new_with_attr_def_ids("".into(), name, description, attr_def_ids);
    log::debug!("Creating an entity definition {:?}: ", ent_def);
    match crate::server::fns::create_entity_def(ent_def).await {
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
