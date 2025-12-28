use crate::ui::{Route, STATE};
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar.
/// The contents of the Home and Blog routes will be rendered under the outlet inside this component.
#[component]
pub fn Navbar() -> Element {
    if !STATE.read().is_ready {
        return rsx! { "" };
    }
    render()
}

fn render() -> Element {
    rsx! {
        nav { class: "absolute w-full px-4 py-2 flex justify-between items-center z-40",
            Link { class: "text-3xl font-bold leading-none", to: Route::Home {}, Logo {} }
            ul { class: "hidden absolute top-1/2 sm:left-1/3 sm:pl-16 md:left-1/2 lg:left-1/2
                    transform -translate-y-1/2 -translate-x-1/2
                    sm:flex sm:mx-auto sm:flex sm:items-center sm:w-auto sm:space-x-3 lg:space-x-6",
                li {
                    Link {
                        class: "text-sm text-gray-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200",
                        to: Route::Home {},
                        "Home"
                    }
                }
                NavSep {}
                li {
                    Link {
                        class: "text-sm text-gray-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200",
                        to: Route::Blog { id: 1 },
                        "Blog"
                    }
                }
            }
            NavUserMenu {}
        }
        div { class: "pt-14", Outlet::<Route> {} }
    }
}

fn NavSep() -> Element {
    rsx! {
        li { class: "text-gray-300",
            div { dangerous_inner_html: r#"
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" class="w-4 h-4 current-fill"
                        viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 5v0m0 7v0m0 7v0m0-13a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 
                               0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
                    </svg>
                "# }
        }
    }
}

const LOGO: Asset = asset!("/assets/logo.png");

fn Logo() -> Element {
    rsx! {
        div {
            img { src: LOGO, alt: "logo", class: "h-8" }
        }
    }
}

fn NavUserMenu() -> Element {
    rsx! {
        div { class: "text-sm text-gray-600", "User Menu" }
    }
}
