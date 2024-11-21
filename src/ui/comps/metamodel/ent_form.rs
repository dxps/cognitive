use crate::domain::model::{BooleanAttribute, Id, IntegerAttribute, Item, ItemType, SmallintAttribute, TextAttribute};
use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(Props, PartialEq, Clone)]
pub struct EntityFormProps {
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
        mut text_attrs,
        mut smallint_attrs,
        mut int_attrs,
        mut boolean_attrs,
        action,
    } = props;

    let is_view = action == "View";

    let ordered_attrs = order_entity_attributes(text_attrs(), smallint_attrs(), int_attrs(), boolean_attrs());

    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "space-y-0",
                //
                for (name , id , item_type) in ordered_attrs {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36 text-gray-600", "{name}:" }
                        if item_type == ItemType::TextAttribute {
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
                                }
                            }
                        } else if item_type == ItemType::SmallintAttribute {
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
                                }
                            }
                        } else if item_type == ItemType::IntegerAttribute {
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
                                }
                            }
                        } else if item_type == ItemType::BooleanAttribute {
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
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Order the attributes of an entity in the alphabetical order,\
/// to be nicely and consistently displayed in the form.
pub fn order_entity_attributes(
    text_attrs: IndexMap<Id, TextAttribute>,
    smallint_attrs: IndexMap<Id, SmallintAttribute>,
    int_attrs: IndexMap<Id, IntegerAttribute>,
    boolean_attrs: IndexMap<Id, BooleanAttribute>,
) -> Vec<(String, Id, ItemType)> {
    //
    let mut ordered_attrs = Vec::new();
    ordered_attrs.extend(
        text_attrs
            .iter()
            .map(|(id, attr)| (attr.name.clone(), id.clone(), attr.item_type())),
    );
    ordered_attrs.extend(
        smallint_attrs
            .iter()
            .map(|(id, attr)| (attr.name.clone(), id.clone(), attr.item_type())),
    );
    ordered_attrs.extend(int_attrs.iter().map(|(id, attr)| (attr.name.clone(), id.clone(), attr.item_type())));
    ordered_attrs.extend(
        boolean_attrs
            .iter()
            .map(|(id, attr)| (attr.name.clone(), id.clone(), attr.item_type())),
    );
    ordered_attrs.sort_by(|attr1, attr2| attr1.0.cmp(&attr2.0));
    ordered_attrs
}
