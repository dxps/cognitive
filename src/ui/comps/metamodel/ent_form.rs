use crate::domain::model::{AttributeValueType, BooleanAttribute, Id, IntegerAttribute, SmallintAttribute, TextAttribute};
use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(Props, PartialEq, Clone)]
pub struct EntityFormProps {
    pub attributes_order: Signal<Vec<(AttributeValueType, Id)>>,
    pub text_attrs: Signal<IndexMap<Id, TextAttribute>>,
    pub smallint_attrs: Signal<IndexMap<Id, SmallintAttribute>>,
    pub int_attrs: Signal<IndexMap<Id, IntegerAttribute>>,
    pub boolean_attrs: Signal<IndexMap<Id, BooleanAttribute>>,
    pub action: String,
}

#[component]
pub fn EntityForm(props: EntityFormProps) -> Element {
    //
    let EntityFormProps {
        attributes_order,
        mut text_attrs,
        mut smallint_attrs,
        mut int_attrs,
        mut boolean_attrs,
        action,
    } = props;

    let is_view = action == "View";

    log::debug!(
        "[EntityForm] attributes_order: {:?} text_attrs: {:?}",
        attributes_order(),
        text_attrs()
    );

    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "space-y-0",
                //
                for (value_type , id) in attributes_order() {
                    div { class: "flex",
                        if value_type == AttributeValueType::Text {
                            label { class: "pr-3 py-2 min-w-36 text-gray-600",
                                "{text_attrs().get(&id).unwrap().name}:"
                            }
                            textarea {
                                key: "{id}",
                                class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                                rows: 1,
                                cols: 32,
                                value: "{text_attrs().get(&id).unwrap().value}",
                                readonly: is_view,
                                maxlength: 256,
                                oninput: move |evt| {
                                    let id = id.clone();
                                    text_attrs
                                        .write()
                                        .entry(id.clone())
                                        .and_modify(|attr| { attr.value = evt.value() });
                                },
                            }
                        } else if value_type == AttributeValueType::SmallInteger {
                            label { class: "pr-3 py-2 min-w-36 text-gray-600",
                                "{smallint_attrs().get(&id).unwrap().name}:"
                            }
                            input {
                                key: "{id}",
                                class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                                r#type: "number",
                                value: "{smallint_attrs().get(&id).unwrap().value}",
                                readonly: is_view,
                                oninput: move |evt| {
                                    let id = id.clone();
                                    smallint_attrs
                                        .write()
                                        .entry(id.clone())
                                        .and_modify(|attr| { attr.value = evt.value().parse().unwrap() });
                                    log::debug!(
                                        "[EntityForm] Changed smallint attr '{:?}' value to '{}'.", smallint_attrs()
                                        .get(& id).unwrap().name, smallint_attrs().get(& id).unwrap().value
                                    );
                                },
                            }
                        } else if value_type == AttributeValueType::Integer {
                            label { class: "pr-3 py-2 min-w-36 text-gray-600",
                                "{int_attrs().get(&id).unwrap().name}:"
                            }
                            input {
                                class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                                r#type: "number",
                                value: "{int_attrs().get(&id).unwrap().value}",
                                readonly: is_view,
                                oninput: move |evt| {
                                    let id = id.clone();
                                    int_attrs
                                        .write()
                                        .entry(id.clone())
                                        .and_modify(|attr| { attr.value = evt.value().parse().unwrap() });
                                    log::debug!(
                                        "[EntityForm] Changed int attr '{:?}' value to '{}'.", int_attrs().get(& id)
                                        .unwrap().name, int_attrs().get(& id).unwrap().value
                                    );
                                },
                            }
                        } else if value_type == AttributeValueType::Boolean {
                            label { class: "pr-3 py-2 min-w-36 text-gray-600",
                                "{boolean_attrs().get(&id).unwrap().name}:"
                            }
                            input {
                                class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                                r#type: "checkbox",
                                checked: "{boolean_attrs().get(&id).unwrap().value}",
                                readonly: is_view,
                                oninput: move |evt| {
                                    let id = id.clone();
                                    boolean_attrs
                                        .write()
                                        .entry(id.clone())
                                        .and_modify(|attr| { attr.value = evt.value().parse().unwrap() });
                                    log::debug!(
                                        "[EntityForm] Changed boolean attr '{:?}' value to '{}'.", boolean_attrs()
                                        .get(& id).unwrap().name, boolean_attrs().get(& id).unwrap().value
                                    );
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
