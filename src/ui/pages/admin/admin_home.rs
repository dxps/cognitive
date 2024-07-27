use dioxus::prelude::*;

use crate::ui::{comps::Nav, routes::Route};

#[component]
pub fn Admin() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3",
                    div { class: "p-6",
                        h5 { class: "mb-2 block text-lg font-semibold leading snug tracking-normal text-gray-500 antialiased",
                            "Data Management"
                        }
                        p { class: "block font-sans text-base font-light leading-relaxed text-inherit antialiased",
                            "Manage your data such as definitions of attributes, entities, and links,
                            plus the instances of these elements."
                        }
                        hr { class: "mt-2 mb-4" }
                        div { class: "flex",
                            div { class: "border-4 border-gray-100 rounded-md p-3 flex flex-col grow mr-1",
                                h6 { class: "mb-2 pl-4 block font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                    "Definitions"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-sm transition duration-200",
                                    to: Route::AttributeDefsPage {},
                                    "Attributes Definitions"
                                }
                                hr { class: "my-2" }
                                Link {
                                    class: "py-2 px-4 rounded-sm transition duration-200",
                                    to: Route::AttributeDefsPage {},
                                    "Entities Definitions"
                                }
                            }
                            div { class: "border-4 border-gray-100 rounded-md p-3 flex flex-col grow ml-1",
                                h6 { class: "mb-2 pl-4 block font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                    "Instances"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-sm transition duration-200",
                                    to: Route::AttributeDefsPage {},
                                    "Attributes Instances"
                                }
                                hr { class: "my-2" }
                                Link {
                                    class: "py-2 px-4 rounded-sm transition duration-200",
                                    to: Route::AttributeDefsPage {},
                                    "Entities Instances"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
