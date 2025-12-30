use crate::be::srvfns::echo_server;
use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside other hooks, async blocks, if statements, or loops.
    //
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }

        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                // `oninput` is an event handler that will run when the input changes. It can return either nothing or a future
                // that will be run when the event runs.
                oninput: move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            // Signals can be called like a function to clone the current value of the signal
            if !response().is_empty() {
                p {
                    "Server echoed: "
                    // Since we read the signal inside this component, the component "subscribes" to the signal. Whenever
                    // the signal changes, the component will rerun.
                    i { "{response}" }
                }
            }
        }
    }
}
