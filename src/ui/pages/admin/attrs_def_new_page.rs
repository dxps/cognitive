use dioxus::prelude::*;
use log::debug;

use crate::ui::{
    comps::{Breadcrumb, Nav},
    routes::Route,
};

#[component]
pub fn AttributeDefNewPage() -> Element {
    //
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut value_type = use_signal(|| "".to_string());
    let mut default_value = use_signal(|| "".to_string());
    let mut is_required = use_signal(|| false);
    let mut is_multivalued = use_signal(|| false);
    let mut tag_id = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create an Attribute Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                        "Fill in the following form to create a new attribute definition."
                        div { class: "mt-4 space-y-4",
                            div { class: "flex",
                                label { class: "pr-3 py-2 min-w-28", "Name:" }
                                input {
                                    class: "px-3 py-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                                    r#type: "text",
                                    placeholder: "its name",
                                    value: "{name}",
                                    maxlength: 64,
                                    autofocus: "true",
                                    oninput: move |evt| {
                                        name.set(evt.value());
                                    },
                                    onmounted: move |evt| async move {
                                        _ = evt.set_focus(true).await;
                                    }
                                }
                            }
                            div { class: "flex",
                                label { class: "pr-3 py-2 min-w-28", "Description:" }
                                textarea {
                                    class: "px-3 py-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                                    rows: 4,
                                    cols: 32,
                                    placeholder: "an optional description",
                                    value: "{name}",
                                    maxlength: 256,
                                    oninput: move |evt| {
                                        description.set(evt.value());
                                    }
                                }
                            }
                            div { class: "flex",
                                label { class: "pr-3 py-1 min-w-28", "Value Type:" }
                                select {
                                    class: "px-3 py-2 bg-slate-100 rounded-lg outline-none border-1 border-gray-300 focus:border-green-300 min-w-80",
                                    multiple: false,
                                    oninput: move |evt| {
                                        value_type.set(evt.value());
                                        log::debug!("selected value type: {:?}", evt.value());
                                    },
                                    option { value: "text", "Text" }
                                    option { value: "smallint", "SmallInteger" }
                                    option { value: "integer", "Integer" }
                                    option { value: "bigint", "BigInteger" }
                                    option { value: "real", "Decimal" }
                                }
                            }
                        }
                        div { class: "flex py-2",
                            label { class: "pr-3 py-2 min-w-28", "Default Value:" }
                            input {
                                class: "px-3 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                                r#type: "text",
                                placeholder: "an optional default value",
                                value: "{default_value}",
                                maxlength: 64,
                                oninput: move |evt| {
                                    default_value.set(evt.value());
                                }
                            }
                        }
                        div { class: "flex py-2",
                            input {
                                class: "px-3 rounded-lg outline-none border-1 focus:border-green-300",
                                r#type: "checkbox",
                                placeholder: "an optional default value",
                                value: "{is_required}",
                                oninput: move |evt| {
                                    is_required.set(evt.value().parse().unwrap_or_default());
                                }
                            }
                            label { class: "pl-3 py-2 min-w-28", "Required" }
                        }
                        div { class: "flex py-2",
                            input {
                                class: "px-3 rounded-lg outline-none border-1 focus:border-green-300",
                                r#type: "checkbox",
                                placeholder: "an optional default value",
                                value: "{is_multivalued}",
                                oninput: move |evt| {
                                    is_multivalued.set(evt.value().parse().unwrap_or_default());
                                }
                            }
                            label { class: "pl-3 min-w-28", "Multivalued" }
                        }
                    }
                }
            }
        }
    }
}
