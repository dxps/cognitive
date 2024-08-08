use crate::{
    domain::model::{Id, Tag},
    server::fns::create_tag,
    ui::{
        comps::{Breadcrumb, Nav, TagForm},
        routes::Route,
        Mode, UI_GLOBAL_SIGNALS,
    },
};
use dioxus::prelude::*;

#[component]
pub fn TagNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::TagNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create a Tag"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-2" }
                        "Fill in the following form to create a new tag."
                        TagForm { name, description, mode: Mode::Edit }
                        div { class: "flex justify-betweent mt-8",
                            // Show the button's action result in the UI.
                            div { class: "min-w-[440px] max-w-[440px]",
                                if err().is_some() {
                                    span { class: "text-red-600", { err().unwrap() } }
                                } else if saved() {
                                    span { class: "text-green-600", { "Successfully created" } }
                                }
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    async move {
                                        if name().is_empty() {
                                            err.set(Some("Name cannot be empty".to_string()));
                                            return;
                                        }
                                        let id = handle_create_tag(name(), description.clone(), saved, err).await;
                                        if id.is_some() {
                                            UI_GLOBAL_SIGNALS
                                                .add_tag(Tag::new(id.unwrap(), name(), description))
                                                .await;
                                        }
                                    }
                                },
                                "Create"
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn handle_create_tag(
    name: String,
    description: Option<String>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    match create_tag(name, description).await {
        Ok(id) => {
            saved.set(true);
            err.set(None);
            Some(id)
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
            None
        }
    }
}
