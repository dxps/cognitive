use std::collections::HashMap;

use dioxus::prelude::*;

use crate::domain::model::{BooleanAttribute, Id, IntegerAttribute, SmallintAttribute, TextAttribute};

#[derive(Props, PartialEq, Clone)]
pub struct EntityFormProps {
    pub text_attrs: Signal<HashMap<Id, (TextAttribute, String)>>,
    pub smallint_attrs: Signal<HashMap<Id, (SmallintAttribute, String)>>,
    pub int_attrs: Signal<HashMap<Id, (IntegerAttribute, String)>>,
    pub boolean_attrs: Signal<HashMap<Id, (BooleanAttribute, String)>>,
    pub action: String,
    pub saved: Signal<bool>,
    pub err: Signal<Option<String>>,
}

#[component]
pub fn EntityForm(props: EntityFormProps) -> Element {
    //
    let EntityFormProps {
        text_attrs,
        smallint_attrs,
        int_attrs,
        boolean_attrs,
        action,
        saved,
        err,
    } = props;

    let is_view = action == "View";

    rsx! {
        div { class: "mt-4 space-y-4",
            // div { class: "flex",
            //     label { class: "pr-3 py-2 min-w-28", "Name:" }
            //     input {
            //         key: "name_{action}",
            //         class: "px-3 py-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
            //         r#type: "text",
            //         value: "{name}",
            //         maxlength: 64,
            //         readonly: is_view,
            //         autofocus: !is_view,
            //         oninput: move |evt| {
            //             name.set(evt.value());
            //         },
            //         onmounted: move |evt| async move {
            //             if !is_view {
            //                 _ = evt.set_focus(true).await;
            //             }
            //         }
            //     }
            // }
            hr {}
            div { class: "space-y-0",
                for (id , (attr , value)) in text_attrs() {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36", "{attr.name}:" }
                        textarea {
                            class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                            rows: 1,
                            cols: 32,
                            value: "{value}",
                            readonly: is_view,
                            maxlength: 256,
                            oninput: move |evt| {
                                let id = id.clone();
                                text_attrs().entry(id).and_modify(|(_, value)| { *value = evt.value() });
                            }
                        }
                    }
                }
                for (id , (attr , value)) in smallint_attrs() {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36", "{attr.name}:" }
                        input {
                            class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                            r#type: "number",
                            value: "{value}",
                            readonly: is_view,
                            maxlength: 3,
                            oninput: move |evt| {
                                let id = id.clone();
                                smallint_attrs().entry(id).and_modify(|(_, value)| { *value = evt.value() });
                            }
                        }
                    }
                }
                for (id , (attr , value)) in int_attrs() {
                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-36", "{attr.name}:" }
                        input {
                            class: "px-3 py-2 my-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                            r#type: "number",
                            value: "{value}",
                            readonly: is_view,
                            maxlength: 10,
                            oninput: move |evt| {
                                let id = id.clone();
                                int_attrs().entry(id).and_modify(|(_, value)| { *value = evt.value() });
                            }
                        }
                    }
                }
            }
            hr { class: "mt-8 mb-1" }
        }
    }
}
