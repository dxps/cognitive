use crate::{
    domain::model::{Id, Tag},
    server::fns::update_tag,
    ui::{
        comps::{Breadcrumb, Nav, TagForm},
        routes::Route,
        Mode, UI_GLOBAL_SIGNALS,
    },
};
use dioxus::prelude::*;

#[component]
pub fn TagPage(id: Id) -> Element {
    //
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let mut mode = use_signal(|| Mode::View);

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    let tid = id.clone();
    use_future(move || {
        let id = tid.clone();
        async move {
            if let Some(t) = UI_GLOBAL_SIGNALS.get_tag(id).await {
                name.set(t.name.clone());
                description.set(t.description.unwrap_or_default());
            }
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_tag(Route::TagPage { id: id.clone() }, name()) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{mode} Tag"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-2" }
                        if mode() == Mode::View {
                            "This tag has the following details:"
                        } else {
                            "Change any of the fields below to update the tag."
                        }
                        TagForm { name, description, mode: mode() }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-400 bg-slate-50 hover:text-red-700 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| { async move { todo!() } },
                                "Delete"
                            }
                            // Show the buttons' action result in the UI.
                            div { class: "min-w-[350px] max-w-[350px] mt-1 pl-2",
                                if err().is_some() {
                                    span { class: "text-red-600", { err().unwrap() } }
                                } else if saved() {
                                    span { class: "text-green-600", { "Successfully created" } }
                                }
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    let id = id.clone();
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    let curr_mode = mode().clone();
                                    async move {
                                        if curr_mode == Mode::View {
                                            mode.set(Mode::Edit);
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            let tag = Tag::new(id, name(), description);
                                            update_handler(tag, saved, err).await;
                                        }
                                    }
                                },
                                if mode() == Mode::View {
                                    "Edit"
                                } else {
                                    "Update"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn update_handler(tag: Tag, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Updating tag: {:?}", tag);
    match update_tag(tag.clone()).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
            UI_GLOBAL_SIGNALS.update_tag(tag).await;
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
