use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`.
#[component]
pub fn HomeView() -> Element {
    rsx! {
        div { class: "pt-[var(--nav-height)] min-h-[calc(100vh-var(--nav-height))] flex",
            div { class: "flex flex-col grow justify-center items-center py-6 drop-shadow-2xl",
                div { class: "bg-white dark:bg-(--dark-bg-d1) rounded-lg p-4 sm:min-w-[600px] sm:min-h-[500px]",
                    h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)",
                        "Home"
                    }
                }
            }
        }
    }
}
