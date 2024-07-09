#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3",
                    div {
                        h1 { class: "text-center text-4xl text-bold", "{count}" }
                        button {
                            class: "bg-slate-200 rounded-lg mr-2 px-2 py-1",
                            onclick: move |_| count += 1,
                            "Up high!"
                        }
                        button {
                            class: "bg-slate-200 rounded-lg m-2 px-2 py-1",
                            onclick: move |_| count -= 1,
                            "Down low!"
                        }
                        Link { class: "pl-2", to: Route::Blog { id: count() }, "Go to Blog {count}" }
                    }
                    hr {}
                    div {
                        button {
                            class: "bg-slate-200 rounded-lg my-2 px-2 py-1",
                            onclick: move |_| async move {
                                if let Ok(data) = get_server_data().await {
                                    println!("Client received: {}", data);
                                    text.set(data.clone());
                                    post_server_data(data).await.unwrap();
                                }
                            },
                            "Get Server Data"
                        }
                        p { "Server data: {text}" }
                    }
                }
            }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
