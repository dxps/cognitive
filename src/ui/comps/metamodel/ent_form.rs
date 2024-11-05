use crate::domain::model::{BooleanAttribute, Id, IntegerAttribute, SmallintAttribute, TextAttribute};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Props, PartialEq, Clone)]
pub struct EntityFormProps {
    pub text_attrs: Signal<HashMap<Id, TextAttribute>>,
    pub smallint_attrs: Signal<HashMap<Id, SmallintAttribute>>,
    pub int_attrs: Signal<HashMap<Id, IntegerAttribute>>,
    pub boolean_attrs: Signal<HashMap<Id, BooleanAttribute>>,
    pub action: String,
    pub err: Signal<Option<String>>,
}

#[component]
pub fn EntityForm(props: EntityFormProps) -> Element {
    //
    let EntityFormProps {
        mut text_attrs,
        mut smallint_attrs,
        mut int_attrs,
        mut boolean_attrs,
        action,
        err,
    } = props;

    let is_view = action == "View";

    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "space-y-0",
                for (id , attr) in text_attrs() {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36", "{attr.name}:" }
                        textarea {
                            class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                            rows: 1,
                            cols: 32,
                            value: "{attr.value}",
                            readonly: is_view,
                            maxlength: 256,
                            oninput: move |evt| {
                                let id = id.clone();
                                text_attrs
                                    .write()
                                    .entry(id.clone())
                                    .and_modify(|attr| { attr.value = evt.value() });
                                log::debug!(
                                    "[EntityForm] After the change, text attr is {:?} and all text_attrs are {:?}",
                                    text_attrs().entry(id), text_attrs()
                                );
                            }
                        }
                    }
                }
                for (id , attr) in smallint_attrs() {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36", "{attr.name}:" }
                        input {
                            class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                            r#type: "number",
                            value: "{attr.value}",
                            readonly: is_view,
                            maxlength: 3,
                            oninput: move |evt| {
                                let id = id.clone();
                                smallint_attrs
                                    .write()
                                    .entry(id)
                                    .and_modify(|attr| {
                                        let value = evt.value();
                                        if let Ok(value) = value.parse::<i8>() {
                                            attr.value = value
                                        } else {
                                            log::warn!("[EntityForm] value {} cannot be parsed as i8", value);
                                        }
                                    });
                            }
                        }
                    }
                }
                for (id , attr) in int_attrs() {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36", "{attr.name}:" }
                        input {
                            class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                            r#type: "number",
                            value: "{attr.value}",
                            readonly: is_view,
                            maxlength: 10,
                            oninput: move |evt| {
                                let id = id.clone();
                                int_attrs
                                    .write()
                                    .entry(id)
                                    .and_modify(|attr| {
                                        let value = evt.value();
                                        if let Ok(value) = value.parse::<i32>() {
                                            attr.value = value
                                        } else {
                                            log::warn!("[EntityForm] value {} cannot be parsed as i32", value);
                                        }
                                    });
                            }
                        }
                    }
                }
                for (id , attr) in boolean_attrs() {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36", "{attr.name}:" }
                        input {
                            class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300",
                            r#type: "checkbox",
                            checked: attr.value,
                            readonly: is_view,
                            oninput: move |evt| {
                                let id = id.clone();
                                boolean_attrs
                                    .write()
                                    .entry(id)
                                    .and_modify(|attr| { attr.value = evt.value() == "true" });
                            }
                        }
                    }
                }
            }
        }
    }
}
