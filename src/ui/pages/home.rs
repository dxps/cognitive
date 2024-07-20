use dioxus::prelude::*;

use crate::{
    domain::model::{ContactPoint, Tag},
    server::fns::test_list_tags,
};

#[component]
pub fn Home() -> Element {
    use crate::ui::ui_global_state::COUNT;
    use crate::{
        server::fns::{get_server_data, post_server_data},
        ui::routes::Route,
    };

    let mut server_data_text = use_signal(|| "".to_string());
    let mut tags_text = use_signal::<Option<Vec<Tag>>>(|| None);

    rsx! {
        div { class: "bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-4 space-y-4",
                    div {
                        h1 { class: "text-center text-4xl text-bold pb-4", "{COUNT}" }
                        button {
                            class: "bg-slate-300 rounded-lg px-2 py-1",
                            onclick: move |_| *COUNT.write() += 1,
                            "Up high!"
                        }
                        button {
                            class: "bg-slate-400 rounded-lg ml-2 px-2 py-1",
                            disabled: COUNT() <= 0,
                            onclick: move |_| *COUNT.write() -= 1,
                            "Down low!"
                        }
                        if COUNT() > 0 {
                            Link { class: "pl-4", to: Route::Blog { id: COUNT() }, "Go to Blog {COUNT}" }
                        } else {
                            span { class: "pl-4 text-slate-500", "Blog {COUNT}" }
                        }
                    }
                    div { class: "pt-4",
                        button {
                            class: "bg-slate-200 rounded-lg px-2 py-1",
                            onclick: move |_| async move {
                                if let Ok(data) = get_server_data().await {
                                    log::debug!(">>> Received from get_server_data: {}", data);
                                    server_data_text.set(data.clone());
                                    post_server_data(data).await.unwrap();
                                }
                            },
                            "Get Server Data"
                        }
                        p { class: "pt-2", "Server data: {server_data_text}" }
                    }
                    div { class: "pt-4",
                        button {
                            class: "bg-slate-200 rounded-lg px-2 py-1",
                            onclick: move |_| async move {
                                if let Ok(tags) = test_list_tags().await {
                                    log::debug!(">>> Received from test_get_contact_point: {:?}", tags);
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
