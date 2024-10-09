use crate::domain::model::Id;

use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    pub items: Signal<HashMap<Id, String>>,
    pub selected_item_id: Signal<Id>,
}

/// The HTML's `select` as a component.<br/>
pub fn Select(props: SelectProps) -> Element {
    //
    let SelectProps {
        items,
        mut selected_item_id,
    } = props;

    rsx! {
        select {
            class: "px-3 my-1 bg-slate-100 rounded-lg outline-none border-1 border-gray-300 focus:border-green-300 min-w-80",
            multiple: false,
            oninput: move |evt| {
                selected_item_id.set(evt.value());
            },
            option { value: "", selected: "{selected_item_id() == \"\"}", "" }
            if !items().is_empty() {
                for (id , kind) in items() {
                    option { value: "{id}", selected: "{selected_item_id() == id}", "{kind}" }
                }
            }
        }
    }
}
