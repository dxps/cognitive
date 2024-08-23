use std::collections::HashMap;

use dioxus::prelude::*;

use crate::domain::model::Id;

#[component]
pub fn EntityDefForm(
    name: Signal<String>,
    description: Signal<String>,
    included_attr_defs: Signal<Vec<(Id, String)>>,
    all_attr_defs: Signal<HashMap<Id, String>>,
    action: String,
    saved: Signal<bool>,
    err: Signal<Option<String>>,
) -> Element {
    //
    let is_view = action == "View";
    let mut selected_attr_def_id = use_signal(|| "".to_string());
    let mut selected_attr_def_name = use_signal(|| "".to_string());

    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Name:" }
                input {
                    key: "name_{action}",
                    class: "px-3 py-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    r#type: "text",
                    value: "{name}",
                    maxlength: 64,
                    readonly: is_view,
                    autofocus: !is_view,
                    oninput: move |evt| {
                        name.set(evt.value());
                    },
                    onmounted: move |evt| async move {
                        if !is_view {
                            _ = evt.set_focus(true).await;
                        }
                    }
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Description:" }
                textarea {
                    class: "px-3 py-2 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    rows: 3,
                    cols: 32,
                    value: "{description}",
                    readonly: is_view,
                    maxlength: 256,
                    oninput: move |evt| {
                        description.set(evt.value());
                    }
                }
            }
            p { "Attributes:" }
            div { class: "space-y-0",
                for (id , name) in included_attr_defs() {
                    div { class: "flex",
                        p { class: "pl-28 pr-3 min-w-[430px]", "{name}" }
                        button {
                            class: "text-red-200 hover:text-red-500 hover:bg-red-100 disabled:text-white disabled:hover:bg-white ml-4 px-3 py-0 rounded-xl transition duration-200",
                            disabled: is_view,
                            // Remove the item from `included_attr_defs` and put it back into `all_attr_defs`.
                            onclick: move |_| {
                                let id = id.clone();
                                let name = name.clone();
                                let mut temp = included_attr_defs();
                                temp.retain(|(iid, _)| *iid != id);
                                included_attr_defs.set(temp);
                                let mut temp = all_attr_defs();
                                temp.insert(id, name);
                                all_attr_defs.set(temp);
                            },
                            "-"
                        }
                    }
                }
            }
            hr { class: "mt-8 mb-1" }
            div {
                class: "flex",
                display: if action == "View" || action == "Delete" || (action == "Edit" && saved()) {
                    "none"
                } else {
                    "block"
                },
                label { class: "pr-3 py-1 min-w-28", "" }
                p { class: "text-gray-500 font-sm",
                    "Select an attribute definition to include it in this entity definition."
                }
                select {
                    class: "px-3 py-2 bg-slate-100 rounded-lg outline-none border-1 border-gray-300 focus:border-green-300 min-w-80",
                    multiple: false,
                    disabled: is_view,
                    oninput: move |evt| {
                        selected_attr_def_id.set(evt.value());
                        selected_attr_def_name
                            .set(all_attr_defs().get(&selected_attr_def_id()).unwrap().to_string());
                    },
                    option { value: "", selected: true, "" }
                    for (id , name) in all_attr_defs() {
                        option { value: "{id}", "{name}" }
                    }
                }
                button {
                    class: "bg-slate-100 text-slate-600 hover:text-gray-800 ml-4 px-3 rounded-lg transition duration-200",
                    disabled: is_view,
                    onclick: move |_| {
                        if selected_attr_def_id().is_empty() {
                            return;
                        }
                        let mut included = included_attr_defs();
                        included.push((selected_attr_def_id(), selected_attr_def_name()));
                        included_attr_defs.set(included);
                        let mut attr_defs = all_attr_defs();
                        attr_defs.remove(&selected_attr_def_id());
                        all_attr_defs.set(attr_defs);
                        selected_attr_def_id.set("".to_string());
                        selected_attr_def_name.set("".to_string());
                        err.set(None);
                    },
                    "+"
                }
            }
        }
    }
}
