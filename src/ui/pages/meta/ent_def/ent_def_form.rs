use crate::domain::model::Id;
use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(Props, PartialEq, Clone)]
pub struct EntityDefFormProps {
    pub name: Signal<String>,
    pub description: Signal<String>,
    pub included_attr_defs: Signal<IndexMap<Id, String>>,
    pub listing_attr_def_id: Signal<Id>,
    pub all_attr_defs: Signal<IndexMap<Id, String>>,
    pub action: String,
    pub action_done: Signal<bool>,
    pub err: Signal<Option<String>>,
}

#[component]
pub fn EntityDefForm(props: EntityDefFormProps) -> Element {
    //
    let EntityDefFormProps {
        mut name,
        mut description,
        mut included_attr_defs,
        mut listing_attr_def_id,
        mut all_attr_defs,
        action,
        action_done,
        mut err,
    } = props;

    let is_view = action == "View";

    let mut selected_attr_def_id = use_signal(|| Id::default());
    let mut selected_attr_def_name = use_signal(|| "".to_string());

    log::debug!("[EntityDefForm] listing_attr_def_id: {:?}", listing_attr_def_id);

    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Name" }
                input {
                    key: "name_{action}",
                    class: "px-3 py-1 min-w-80",
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
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Description" }
                textarea {
                    class: "px-3 py-2 min-w-80",
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
            div { class: "flex",
                p { class: "min-w-32 text-gray-500", "Attributes" }
                div {
                    for (id , name) in included_attr_defs() {
                        div { class: "flex justify-between min-w-80",
                            p { class: "pl-3 pr-3", "{name}" }
                            button {
                                class: "text-red-200 hover:text-red-500 hover:bg-red-100 disabled:text-white disabled:hover:bg-white ml-4 px-3 py-0 rounded-xl transition duration-200",
                                display: if is_view { "none" } else { "inline" },
                                // Remove the item from `included_attr_defs` and put it back into `all_attr_defs`.
                                onclick: move |_| {
                                    let id = id.clone();
                                    let name = name.clone();
                                    let mut temp = included_attr_defs();
                                    temp.swap_remove(&id);
                                    included_attr_defs.set(temp);
                                    let mut temp = all_attr_defs();
                                    temp.insert(id.clone(), name);
                                    all_attr_defs.set(temp);
                                    if listing_attr_def_id() == id {
                                        listing_attr_def_id.set(Id::default());
                                    }
                                },
                                "-"
                            }
                        }
                    }
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Listing attribute" }
                select {
                    class: "px-3 py-2 min-w-80",
                    multiple: false,
                    disabled: is_view,
                    oninput: move |evt| {
                        listing_attr_def_id.set(evt.value().into());
                        log::debug!("[EntityDefForm] selected_attr_def_id: {:?}", evt.value());
                    },
                    for (id , name) in included_attr_defs() {
                        option {
                            value: "{id}",
                            selected: "{listing_attr_def_id() == id}",
                            "{name}"
                        }
                    }
                }
            }
            hr { class: "mt-8 mb-1" }
            div {
                class: "flex",
                display: if action == "View" || action == "Delete" || (action == "Edit" && action_done()) {
                    "none"
                } else {
                    "block"
                },
                label { class: "pr-3 py-1 min-w-28", "" }
                p { class: "text-gray-500 font-sm",
                    "Select an attribute definition to include it in this entity definition."
                }
                select {
                    class: "px-3 py-2 min-w-80",
                    multiple: false,
                    disabled: is_view,
                    oninput: move |evt| {
                        selected_attr_def_id.set(evt.value().into());
                        selected_attr_def_name
                            .set(all_attr_defs().get(&selected_attr_def_id()).unwrap().to_string());
                    },
                    option { value: "", selected: true, "" }
                    for (id , name) in all_attr_defs() {
                        option {
                            value: "{id}",
                            selected: "{selected_attr_def_id() == id}",
                            "{name}"
                        }
                    }
                }
                button {
                    class: "bg-slate-100 text-slate-600 hover:text-gray-800 ml-4 px-3 rounded-lg transition duration-200",
                    disabled: is_view,
                    onclick: move |_| {
                        if selected_attr_def_id().is_empty() {
                            return;
                        }
                        if listing_attr_def_id().is_empty() {
                            listing_attr_def_id.set(selected_attr_def_id());
                        }
                        let mut included = included_attr_defs();
                        included.insert(selected_attr_def_id(), selected_attr_def_name());
                        included_attr_defs.set(included);
                        let mut attr_defs = all_attr_defs();
                        attr_defs.swap_remove(&selected_attr_def_id());
                        all_attr_defs.set(attr_defs);
                        selected_attr_def_id.set(Id::default());
                        selected_attr_def_name.set("".to_string());
                        err.set(None);
                    },
                    "+"
                }
            }
        }
    }
}
