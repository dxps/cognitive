use dioxus::prelude::*;

#[component]
pub fn TagForm(name: Signal<String>, description: Signal<String>, mode: String) -> Element {
    //
    let is_view_mode = mode == "View";
    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Name:" }
                input {
                    key: "name_{mode}",
                    class: "px-3 py-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    r#type: "text",
                    name: "name_{mode}",
                    value: "{name}",
                    maxlength: 64,
                    readonly: is_view_mode,
                    autofocus: !is_view_mode,
                    oninput: move |evt| {
                        name.set(evt.value());
                    },
                    onmounted: move |evt| async move {
                        if !is_view_mode {
                            _ = evt.set_focus(true).await;
                        }
                    }
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Description:" }
                textarea {
                    class: "px-3 py-2 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    rows: 4,
                    cols: 32,
                    placeholder: "an optional description",
                    value: "{description}",
                    readonly: is_view_mode,
                    maxlength: 256,
                    oninput: move |evt| {
                        description.set(evt.value());
                    }
                }
            }
        }
    }
}
