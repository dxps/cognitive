use dioxus::prelude::*;

use crate::{domain::model::Tag, server::fns::tags::get_tags, ui::comps::Nav};

#[component]
pub fn Home() -> Element {
    //
    let mut tags_text = use_signal::<Option<Vec<Tag>>>(|| None);

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-4 space-y-4 max-w-xl",
                    div { class: "pt-4",
                        button {
                            class: "bg-slate-200 rounded-lg px-2 py-1",
                            onclick: move |_| async move {
                                if let Ok(tags) = get_tags().await {
                                    log::debug!(">>> Received from test_list_tags: {:?}", tags);
                                    tags_text.set(Some(tags));
                                }
                            },
                            "Test Get Tags"
                        }
                        p { class: "pt-2", "Tags: {tags_text:?}" }
                    }
                }
            }
        }
    }
}
