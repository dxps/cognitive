use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    pub title: String,
    pub content: String,
    pub children: Element,
}

pub fn Modal(props: ModalProps) -> Element {
    let ModalProps {
        title,
        content,
        children,
    } = props;
    rsx! {
        div { class: "fixed inset-0 p-4 flex flex-wrap justify-center items-center w-full h-full z-[1000] before:fixed before:inset-0 before:w-full before:h-full before:bg-[rgba(0,0,0,0.5)] overflow-auto font-[sans-serif]",
            div { class: "w-full max-w-lg bg-white shadow-lg rounded-lg p-8 relative",
                div {
                    h4 { class: "text-sm text-gray-800 font-semibold", {title} }
                    p { class: "text-sm text-gray-600 mt-4", { content } }
                }
                { children }
            }
        }
    }
}
