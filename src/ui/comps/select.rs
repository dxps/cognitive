use crate::{domain::model::Id, ui::pages::Name};

use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    pub items: Signal<HashMap<Id, Name>>,
    pub selected_item_id: Signal<Id>,
}

/// The HTML's `select` as a component.<br/>
pub fn Select(props: SelectProps) -> Element {
    //
    let SelectProps {
        items,
        mut selected_item_id,
    } = props;

    log::debug!("[Select] on render, selected_item_id: {:?}", selected_item_id());

    rsx! {
        select {
            class: "px-3 my-1 min-w-80",
            multiple: false,
            oninput: move |evt| {
                selected_item_id.set(evt.value().into());
                log::debug!("[Select] set selected_item_id: {:?}", evt.value());
            },
            option { value: "", selected: "{selected_item_id().is_empty()}", "" }
            if !items().is_empty() {
                for (id , kind) in items() {
                    option { value: "{id}", selected: "{selected_item_id() == id}", "{kind}" }
                }
            }
        }
    }
}
