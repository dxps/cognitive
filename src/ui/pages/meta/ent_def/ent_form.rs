use std::collections::HashMap;

use dioxus::prelude::*;

use crate::domain::model::{BooleanAttribute, Id, SmallintAttribute, TextAttribute};

#[derive(Props, PartialEq, Clone)]
pub struct EntityFormProps {
    pub kind: Signal<String>,
    pub text_attrs: Signal<HashMap<Id, (TextAttribute, String)>>,
    pub smallint_attrs: Signal<HashMap<Id, (SmallintAttribute, String)>>,
    pub boolean_attrs: Signal<HashMap<Id, (BooleanAttribute, String)>>,
    pub action: String,
    pub saved: Signal<bool>,
    pub err: Signal<Option<String>>,
}

#[component]
pub fn EntityForm(props: EntityFormProps) -> Element {
    //
    let action = props.action;
    let text_attrs = props.text_attrs;
    let _saved = props.saved;
    let _err = props.err;

    let is_view = action == "View";
    //let mut text_attr_values = use_signal(|| Vec::<String>::new());
    //let mut smallint_attr_values = use_signal(|| Vec::<i8>::new());

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
            p { "Text Attributes:" }
            div { class: "space-y-0",
                for (id , (attr , value)) in text_attrs() {

                    div { class: "flex",
                        label { class: "pr-3 py-2 min-w-28", "{attr.name}:" }
                        textarea {
                            class: "px-3 py-2 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
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
            }
            hr { class: "mt-8 mb-1" }
        }
    }
}
