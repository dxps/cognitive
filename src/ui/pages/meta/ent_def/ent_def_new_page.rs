use std::collections::HashMap;

use crate::{
    domain::model::Id,
    server::fns::list_attribute_defs,
    ui::{
        comps::{Breadcrumb, Nav},
        pages::EntityDefForm,
        routes::Route,
        Action,
    },
};
use dioxus::prelude::*;

pub fn EntityDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let included_attr_defs: Signal<Vec<(Id, String)>> = use_signal(|| vec![]);
    let mut all_attr_defs: Signal<HashMap<Id, String>> = use_signal(|| HashMap::new());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityDefListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                        EntityDefForm {
                            name,
                            description,
                            included_attr_defs,
                            all_attr_defs,
                            action: Action::Edit
                        }
                    }
                }
            }
        }
    }
}

async fn fetch_all_attr_defs() -> HashMap<Id, String> {
    //
    let mut entries = HashMap::new();
    if let Ok(attr_defs) = list_attribute_defs().await {
        attr_defs.iter().for_each(|attr_def| {
            entries.insert(attr_def.id.clone(), attr_def.name.clone());
        });
    }
    entries
}
