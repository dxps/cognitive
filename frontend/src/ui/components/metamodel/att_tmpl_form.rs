use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone, Debug)]
pub struct AttrTemplateFormProps {
    pub name: Signal<String>,
    pub description: Signal<String>,
    pub value_type: Signal<String>,
    pub default_value: Signal<String>,
    pub is_required: Signal<bool>,
    pub action: String,
}

#[component]
pub fn AttributeTemplateForm(props: AttrTemplateFormProps) -> Element {
    //
    let AttrTemplateFormProps {
        mut name,
        mut description,
        mut value_type,
        mut default_value,
        mut is_required,
        action,
    } = props;

    let is_view = action == "View";
    let is_edit = action == "Edit";

    rsx! {
        div { class: "mt-8 space-y-4",
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28 text-gray-500", "Name" }
                input {
                    class: "min-w-80",
                    r#type: "text",
                    value: "{name}",
                    maxlength: 64,
                    readonly: is_view,
                    autofocus: !is_edit,
                    oninput: move |evt| {
                        name.set(evt.value());
                    },
                    onmounted: move |evt| async move {
                        _ = evt.set_focus(is_edit).await;
                    },
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28 text-gray-500", "Description" }
                textarea {
                    class: "min-w-80",
                    rows: 4,
                    cols: 32,
                    placeholder: if action == "Create" { "an optional description" } else { "" },
                    value: "{description}",
                    maxlength: 256,
                    readonly: is_view,
                    oninput: move |evt| {
                        description.set(evt.value());
                    },
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-1 min-w-28 text-gray-500", "Value Type" }
                select {
                    class: "px-3 min-w-80 outline-none",
                    multiple: false,
                    disabled: is_view || is_edit,
                    oninput: move |evt| {
                        value_type.set(evt.value());
                        log::debug!("selected value type: {:?}", evt.value());
                    },
                    option { value: "text", selected: "{value_type() == \"text\"}", "Text" }
                    option {
                        value: "numeric",
                        selected: "{value_type() == \"numeric\"}",
                        "Numeric"
                    }
                    option {
                        value: "boolean",
                        selected: "{value_type() == \"boolean\"}",
                        "Boolean"
                    }
                    option { value: "date", selected: "{value_type() == \"date\"}", "Date" }
                    option {
                        value: "timestamp",
                        selected: "{value_type() == \"timestamp\"}",
                        "Timestamp"
                    }
                }
                if action == "Edit" {
                    div { class: "group flex relative",
                        span { class: "flex text-xs text-gray-400 hover:text-gray-600 cursor-pointer pl-2 items-center",
                            "🛈"
                        }
                        span { class: "group-hover:opacity-100 transition-opacity bg-gray-500 px-1 text-sm text-white rounded-md opacity-0 m-8 py-2 mx-auto absolute right-0 w-48 text-center",
                            "The value type cannot be changed."
                        }
                    }
                }
            }
            div { class: "flex py-2",
                label { class: "pr-3 py-1 min-w-28 text-gray-500", "Default Value" }
                if value_type() != "boolean" {
                    input {
                        class: "min-w-80",
                        r#type: "text",
                        placeholder: if action == "Create" { "an optional default value" } else { "" },
                        value: "{default_value()}",
                        maxlength: 64,
                        readonly: is_view,
                        oninput: move |evt| {
                            default_value.set(evt.value());
                        },
                    }
                } else {
                    input {
                        r#type: "checkbox",
                        checked: default_value(),
                        readonly: is_view,
                        oninput: move |evt| {
                            default_value.set(evt.value());
                        },
                    }
                }
            }
            div { class: "flex items-center",
                label {
                    class: "pr-3 py-1 min-w-28 text-gray-500",
                    cursor: if is_edit { "pointer" } else { "default" },
                    r#for: "is_required",
                    onclick: move |_| {
                        if is_edit {
                            is_required.set(!is_required());
                        }
                    },
                    "Is Required ?"
                }
                input {
                    r#type: "checkbox",
                    id: "is_required",
                    value: "{is_required()}",
                    checked: "{is_required()}",
                    disabled: is_view,
                    oninput: move |evt| {
                        if is_edit {
                            is_required.set(evt.value().parse().unwrap_or_default());
                        }
                    },
                }
            }
        }
    }
}
