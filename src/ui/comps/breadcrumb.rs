use crate::ui::{comps::GtSep, routes::Route};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct BreadcrumbProps {
    pub paths: Vec<(String, Route)>,
}

pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    //
    let last_idx = props.paths.len() - 1;
    rsx! {
        ul { class: "flex items-center justify-center font-[sans-serif] space-x-3 mt-16 mb-4",
            for (i , (label , route)) in props.paths.into_iter().enumerate() {
                Link { class: "text-gray-500 text-xs cursor-pointer", to: route, "{label}" }
                if i < last_idx {
                    GtSep {}
                }
            }
        }
    }
}
