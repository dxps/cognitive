use crate::{
    domain::model::{Id, Tag},
    server::fns::{remove_tag, update_tag},
    ui::{
        comps::{Breadcrumb, Nav, TagForm},
        routes::Route,
        Action, UI_GLOBALS,
    },
};
use dioxus::prelude::*;

#[component]
pub fn TagPage(id: Id) -> Element {
    //
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let mut action = use_signal(|| Action::View);

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let saved = use_signal(|| false);

    let tid = id.clone();
    let did = id.clone();
    use_future(move || {
        let id = tid.clone();
        async move {
            if let Some(t) = UI_GLOBALS.get_tag(id).await {
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
                                "{action} Tag"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagListPage {},
                                "X"
                            }
                        }
                        hr { class: "pb-2" }
                        if action() == Action::View {
                            "This tag has the following details:"
                        } else {
                            "Change any of the fields below to update the tag."
                        }
                        TagForm { name, description, action: action() }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-400 bg-slate-50 hover:text-red-700 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    let id = did.clone();
                                    action.set(Action::Delete);
                                    async move { handle_delete(id, saved, err).await }
                                },
                                "Delete"
                            }
                            // Show the buttons' action result in the UI.
                            div { class: "min-w-[350px] max-w-[350px] mt-1 pl-2",
                                if err().is_some() {
                                    span { class: "text-red-600 flex justify-center",
                                        { err().unwrap() }
                                    }
                                } else if saved() {
                                    span { class: "text-green-600 flex justify-center",
                                        {
                                            if action() == Action::Edit {
                                                "Successfully updated"
                                            } else if action() == Action::Delete {
                                                "Successfully deleted"
                                            } else {
                                                ""
                                            }
                                        }
                                    }
                                }
                            }
                            button {
                                class: "bg-gray-100 enabled:hover:bg-green-100 disabled:text-gray-400 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: action() == Action::Delete,
                                onclick: move |_| {
                                    let id = id.clone();
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    let curr_action = action().clone();
                                    async move {
                                        if curr_action == Action::View {
                                            action.set(Action::Edit);
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            let tag = Tag::new(id, name(), description);
                                            handle_update(tag, saved, err).await;
                                        }
                                    }
                                },
                                if action() == Action::View {
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

async fn handle_update(tag: Tag, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Updating tag: {:?}", tag);
    match update_tag(tag.clone()).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
            UI_GLOBALS.update_tag(tag).await;
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
}

async fn handle_delete(id: String, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Deleting tag: {:?}", id);
    match remove_tag(id.clone()).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
            UI_GLOBALS.remove_tag(id).await;
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
