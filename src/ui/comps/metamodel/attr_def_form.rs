use crate::domain::model::{Id, Tag};
use dioxus::prelude::*;
use std::sync::Arc;

#[derive(Props, PartialEq, Clone)]
pub struct AttributeDefFormProps {
    pub name: Signal<String>,
    pub description: Signal<String>,
    pub value_type: Signal<String>,
    pub default_value: Signal<String>,
    pub is_required: Signal<bool>,
    pub is_multivalued: Signal<bool>,
    pub tag_id: Signal<Id>,
    pub tags: Arc<Vec<Tag>>,
    pub action: String,
}

#[component]
pub fn AttributeDefForm(props: AttributeDefFormProps) -> Element {
    //
    let AttributeDefFormProps {
        mut name,
        mut description,
        mut value_type,
        mut default_value,
        mut is_required,
        mut is_multivalued,
        mut tag_id,
        tags,
        action,
    } = props;

    let is_view = action == "View";
    let is_edit = action == "Edit";
    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Name:" }
                input {
                    class: "px-3 py-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    r#type: "text",
                    placeholder: "its name",
                    value: "{name}",
                    maxlength: 64,
                    readonly: is_view,
                    autofocus: !is_view,
                    oninput: move |evt| {
                        name.set(evt.value());
                    },
                    onmounted: move |evt| async move {
                        _ = evt.set_focus(true).await;
                    }
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Description:" }
                textarea {
                    class: "px-3 py-2 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    rows: 4,
                    cols: 32,
                    placeholder: "an optional description",
                    value: "{description}",
                    maxlength: 256,
                    readonly: is_view,
                    oninput: move |evt| {
                        description.set(evt.value());
                    }
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-1 min-w-28", "Value Type:" }
                select {
                    class: "px-3 py-2 min-w-80",
                    multiple: false,
                    disabled: is_view,
                    oninput: move |evt| {
                        value_type.set(evt.value());
                        log::debug!("selected value type: {:?}", evt.value());
                    },
                    option { value: "text", selected: "{value_type() == \"text\"}", "Text" }
                    option {
                        value: "smallint",
                        selected: "{value_type() == \"smallint\"}",
                        "Small Integer"
                    }
                    option {
                        value: "integer",
                        selected: "{value_type() == \"integer\"}",
                        "Integer"
                    }
                    option {
                        value: "bigint",
                        selected: "{value_type() == \"bigint\"}",
                        "Big Integer"
                    }
                    option { value: "real", selected: "{value_type() == \"real\"}", "Decimal" }
                }
            }
            div { class: "flex py-2",
                label { class: "pr-3 py-2 min-w-28", "Default Value:" }
                input {
                    class: "px-3 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    r#type: "text",
                    placeholder: "an optional default value",
                    value: "{default_value}",
                    maxlength: 64,
                    readonly: is_view,
                    oninput: move |evt| {
                        default_value.set(evt.value());
                    }
                }
            }
            div { class: "flex ",
                input {
                    class: "px-3 rounded-lg outline-none border-1 focus:border-green-300",
                    r#type: "checkbox",
                    value: "{is_required}",
                    checked: "{is_required()}",
                    disabled: is_view,
                    oninput: move |evt| {
                        if is_edit {
                            is_required.set(evt.value().parse().unwrap_or_default());
                        }
                    }
                }
                label {
                    class: "pl-3 py-2 min-w-28",
                    cursor: if is_edit { "pointer" } else { "default" },
                    onclick: move |_| {
                        if is_edit {
                            is_required.set(!is_required());
                        }
                    },
                    "Required"
                }
            }
            div { class: "flex",
                input {
                    class: "px-3 rounded-lg outline-none border-1 focus:border-green-300",
                    r#type: "checkbox",
                    value: "{is_multivalued}",
                    checked: "{is_multivalued()}",
                    disabled: is_view,
                    oninput: move |evt| {
                        is_multivalued.set(evt.value().parse().unwrap_or_default());
                    }
                }
                label {
                    class: "pl-3 min-w-28",
                    cursor: if is_edit { "pointer" } else { "default" },
                    onclick: move |_| {
                        if is_edit {
                            is_multivalued.set(!is_multivalued())
                        }
                    },
                    "Multivalued"
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-1 min-w-28", "Tag:" }
                select {
                    class: "px-3 py-2 min-w-80",
                    multiple: false,
                    disabled: is_view,
                    oninput: move |evt| {
                        tag_id.set(evt.value().into());
                        log::debug!("selected tag_id: {:?}", evt.value());
                    },
                    option { value: "", "" }
                    for tag in tags.iter() {
                        option {
                            value: "{tag.id}",
                            selected: "{tag_id() == tag.id}",
                            "{tag.name}"
                        }
                    }
                }
            }
        }
    }
}
