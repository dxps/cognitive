use dioxus::prelude::*;

use crate::{
    domain::model::{Id, Tag},
    server::fns::update_tag,
    ui::{
        comps::{Breadcrumb, Nav, TagForm},
        routes::Route,
        UI_GLOBAL_SIGNALS,
    },
};

#[component]
pub fn TagEditPage(id: Id) -> Element {
    //
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    let tid = id.clone();
    use_future(move || {
        let id = tid.clone();
        async move {
            if let Some(t) = UI_GLOBAL_SIGNALS.get_tag(id).await {
                log::debug!(">>> Got tag: {:?}", t);
                name.set(t.name.clone());
                description.set(t.description.unwrap_or_default());
            }
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::TagListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Edit a Tag"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagListPage {},
                                "x"
                            }
                        }
                        hr { class: "pb-2" }
                        "Change any of the fields below to update the tag."
                        TagForm { name, description }
                        div { class: "text-center my-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| {
                                    let id = id.clone();
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    async move {
                                        let tag = Tag::new(id, name(), description);
                                        update_handler(tag, saved, err).await;
                                    }
                                },
                                "Update"
                            }
                        }
                        // Show the button's action result in the UI.
                        if err().is_some() {
                            div { class: "text-center text-red-600 my-8",
                                span { {err().unwrap()} }
                            }
                        } else if saved() {
                            div { class: "text-center text-green-600 my-8",
                                span { { "Successfully created" } }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn update_handler(tag: Tag, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    match update_tag(tag).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
