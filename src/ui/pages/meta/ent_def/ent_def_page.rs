use dioxus::prelude::*;

use crate::{
    domain::model::Id,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        Action,
    },
};

#[derive(PartialEq, Props, Clone)]
pub struct EntityDefPageProps {
    id: Id,
}

#[component]
pub fn EntityDefPage(props: EntityDefPageProps) -> Element {
    //
    let id = props.id;
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let mut action = use_signal(|| Action::View);
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_ent_def(Route::EntityDefPage { id }, name()) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Attribute Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                    }
                }
            }
        }
    }
}
