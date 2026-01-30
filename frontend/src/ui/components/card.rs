use dioxus::prelude::*;

#[component]
pub fn Card(header: Element, content: Element) -> Element {
    //
    rsx! {
        div { class: "pt-[var(--nav-height)] min-h-[calc(100vh-var(--nav-height))] flex",
            div { class: "flex flex-col grow justify-center items-center py-6 drop-shadow-2xl",
                div { class: "bg-white dark:bg-(--dark-bg-d1) rounded-lg p-4 sm:min-w-[600px] sm:min-h-[500px]",
                    {header}
                    {content}
                }
            }
        }
    }
}
