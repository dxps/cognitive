use crate::ui::pages::{Admin, AttributeDefsPage, Home, Login, Logout, UserProfile};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/logout")]
    Logout {},

    #[route("/users/:username")]
    UserProfile { username: String },

    #[route("/admin")]
    Admin {},

    #[route("/admin/definitions/attributes")]
    AttributeDefsPage {},
}

impl Route {
    pub fn get_path(to: Route) -> Vec<(String, Route)> {
        match to {
            Route::Home {} => vec![("Home".into(), Route::Home {})],
            Route::Login {} => vec![("Home".into(), to), ("Login".into(), Route::Login {})],
            Route::Logout {} => vec![("Home".into(), Route::Home {}), ("Logout".into(), to)],
            Route::UserProfile { username: _ } => vec![("Home".into(), Route::Home {}), ("User Profile".into(), to)],
            Route::Admin {} => vec![("Home".into(), Route::Home {}), ("Admin".into(), to)],
            Route::AttributeDefsPage {} => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Attributes Definitions".into(), to),
            ],
        }
    }
}
