use crate::ui::comps::nav::common::style_nav_item_link;
use crate::ui::comps::NavUserMenu;
use crate::ui::routes::Route;
use dioxus::prelude::*;

pub fn Nav(props: NavProps) -> Element {
    //
    rsx! {
        nav { class: "absolute w-full px-4 py-3 flex justify-between items-center bg-white z-40",
            Link { class: "text-3xl font-bold leading-none", to: Route::Home {}, Logo {} }
            ul { class: "hidden absolute top-1/2 sm:left-1/3 sm:pl-16 md:left-1/2 lg:left-1/2
                        transform -translate-y-1/2 -translate-x-1/2
                        sm:flex sm:mx-auto sm:flex sm:items-center sm:w-auto sm:space-x-3 lg:space-x-6",
                li {
                    Link {
                        class: style_nav_item_link(&props.active_path, NavProps::home()),
                        to: Route::Home {},
                        "Home"
                    }
                }
                NavSep {}
            }
            NavUserMenu { active_path: &props.active_path }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct NavProps {
    #[props(default = "home".to_string())]
    pub active_path: String,
}

impl NavProps {
    pub fn home() -> String {
        "home".to_string()
    }
    pub fn blog() -> String {
        "blog".to_string()
    }
    pub fn sample() -> String {
        "sample".to_string()
    }
    pub fn login() -> String {
        "login".to_string()
    }
    pub fn users_section() -> String {
        "/users/".to_string()
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

fn Logo() -> Element {
    rsx! {
        div {
            img { src: "/logo.png", alt: "logo", class: "h-9" }
        }
    }
}
