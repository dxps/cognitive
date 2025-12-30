use dioxus::prelude::*;

use crate::ui::{APP_LOCALSTORAGE_KEY, Route, STATE, UiState, UiStorage};

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller.
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete.
#[component]
pub fn App() -> Element {
    use_future(|| async {
        let mut state = STATE.write();

        let storage: UiStorage<UiState> = UiStorage::new(APP_LOCALSTORAGE_KEY).unwrap_or_default();
        if let Some(d) = storage.data {
            debug!("Loaded state from storage: {:#?}", d);
            state.is_light_theme = d.is_light_theme;
            state.user = d.user;
        } else {
            debug!("No state found in storage.");
        }

        let document = web_sys::window().unwrap().document().unwrap();
        let root = document.document_element().unwrap();

        if state.is_light_theme {
            root.class_list().remove_1("dark").unwrap();
        } else {
            root.class_list().add_1("dark").unwrap();
        }

        state.is_ready = true;
    });

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // Add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL
        // and render the layouts and components for the active route.
        Router::<Route> {}
    }
}
