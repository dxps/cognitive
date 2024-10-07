use std::collections::HashMap;

use crate::domain::model::Id;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    pub items: Signal<HashMap<Id, String>>,
    pub selected_item_id: Signal<Id>,
}

/// The HTML's `select` as a component.<br/>
pub fn Select(props: SelectProps) -> Element {
    let items = props.items;
    let items = &items();
    rsx! {
        select {
            class: "px-3 bg-slate-100 rounded-lg outline-none border-1 border-gray-300 focus:border-green-300 min-w-80",
            multiple: false,
            oninput: move |evt| {
                log::debug!("selected_item_id: {:?}", evt.value());
                log::debug!("evt: {:?}", evt);
            },
            option { value: "", selected: true, "" }
            if !items.is_empty() {
                for (id , kind) in items {
                    option { value: "{id}", "{kind}" }
                }
            }
        }
    }
}
