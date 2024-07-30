use crate::ui::pages::{Admin, AttributeDefListPage, AttributeDefNewPage, Home, Login, Logout, UserProfile};
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
    AttributeDefListPage {},

    #[route("/admin/definitions/attributes/new")]
    AttributeDefNewPage {},
}

impl Route {
    pub fn get_path(to: Route) -> Vec<(String, Route)> {
        match to {
            Route::Home {} => vec![("Home".into(), Route::Home {})],
            Route::Login {} => vec![("Home".into(), to), ("Login".into(), Route::Login {})],
            Route::Logout {} => vec![("Home".into(), Route::Home {}), ("Logout".into(), to)],
            Route::UserProfile { username: _ } => vec![("Home".into(), Route::Home {}), ("User Profile".into(), to)],
            Route::Admin {} => vec![("Home".into(), Route::Home {}), ("Admin".into(), to)],
            Route::AttributeDefListPage {} => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Attributes Definitions".into(), to),
            ],
            Route::AttributeDefNewPage {} => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Attributes Definitions".into(), Route::AttributeDefListPage {}),
                ("New".into(), to),
            ],
        }
    }
}
