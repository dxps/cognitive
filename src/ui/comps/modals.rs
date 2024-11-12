use dioxus::prelude::*;

use crate::ui::Action;

#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    pub title: String,
    pub content: String,
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    //
    let ModalProps { title, content, children } = props;

    rsx! {
        div { class: "fixed inset-0 p-4 flex flex-wrap justify-center items-center w-full h-full z-[1000] before:fixed before:inset-0 before:w-full before:h-full before:bg-[rgba(0,0,0,0.5)] overflow-auto",
            div { class: "w-full max-w-lg bg-white shadow-lg rounded-lg p-8 relative",
                div {
                    h4 { class: "text-sm text-gray-800 font-semibold", {title} }
                    p { class: "text text-gray-600 mt-4", { content } }
                }
                { children }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AcknowledgeModalProps {
    pub title: String,
    pub content: Vec<String>,
    pub action_handler: EventHandler,
}

#[component]
pub fn AcknowledgeModal(props: AcknowledgeModalProps) -> Element {
    //
    let AcknowledgeModalProps {
        title,
        content,
        action_handler,
    } = props;

    rsx! {
        div { class: "fixed inset-0 p-4 flex flex-wrap justify-center items-center w-full h-full z-[1000] before:fixed before:inset-0 before:w-full before:h-full before:bg-[rgba(0,0,0,0.5)] overflow-auto",
            div { class: "w-full max-w-lg bg-white shadow-lg rounded-lg p-8 relative",
                div {
                    h4 { class: "text-sm text-gray-800 font-semibold mb-8", {title} }
                    for stmt in content {
                        p { class: "text text-gray-600", { stmt } }
                    }
                }
                div { class: "flex justify-center mt-8",
                    button {
                        class: "bg-gray-100 bg-green-100 enabled:hover:bg-green-100 disabled:text-gray-400 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                        onclick: move |_| {
                            action_handler(());
                        },
                        "OK"
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ConfirmationModalProps {
    pub title: String,
    pub content: String,
    pub action: Signal<Action>,
    pub show_modal: Signal<bool>,
    pub action_handler: EventHandler,
}

#[component]
pub fn ConfirmationModal(props: ConfirmationModalProps) -> Element {
    //
    let ConfirmationModalProps {
        title,
        content,
        mut action,
        mut show_modal,
        action_handler,
    } = props;

    rsx! {
        div { class: "fixed inset-0 p-4 flex flex-wrap justify-center items-center w-full h-full z-[1000] before:fixed before:inset-0 before:w-full before:h-full before:bg-[rgba(0,0,0,0.5)] overflow-auto",
            div { class: "w-full max-w-lg bg-white shadow-lg rounded-lg p-8 relative",
                div {
                    h4 { class: "text-sm text-gray-800 font-semibold", {title} }
                    p { class: "text text-gray-600 mt-8", { content } }
                }
                div { class: "flex justify-between mt-8",
                    button {
                        class: "text-red-600 bg-red-50 hover:text-red-800 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                        onclick: move |_| {
                            show_modal.set(false);
                            action.set(Action::Delete);
                            action_handler(());
                        },
                        "Delete"
                    }
                    button {
                        class: "bg-gray-100 bg-green-100 enabled:hover:bg-green-100 disabled:text-gray-400 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                        onclick: move |_| {
                            show_modal.set(false);
                        },
                        "Cancel"
                    }
                }
            }
        }
    }
}
