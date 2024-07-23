use dioxus::prelude::*;

use crate::ui::{
    comps::{Nav, NavProps},
    routes::Route,
};

#[component]
pub fn Logout() -> Element {
    //
    use_future(move || async move {
        handle_logout().await;
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav { active_path: NavProps::blog() }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3",
                    div { class: "text-xl px-2", "Have a great day! See you later!" }
                }
                div { class: "pt-8",
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        }
    }
}

async fn handle_logout() {
    use crate::server::fns::auth::logout;
    use crate::ui::State;

    log::debug!(">>> Handling the logout ...");
    logout().await.unwrap(); // TODO: Handle this if it fails.
    let mut state = State::new().unwrap();
    state.save_to_localstorage();
    let mut state_sgnl = use_context::<Signal<State>>();
    *state_sgnl.write() = state;
}
