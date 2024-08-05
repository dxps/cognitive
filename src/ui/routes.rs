use crate::{
    domain::model::Id,
    ui::pages::{
        Admin, AttributeDefEditPage, AttributeDefListPage, AttributeDefNewPage, Home, Login, Logout, TagListPage, TagNewPage,
        TagPage, UserProfile,
    },
};
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

    #[route("/admin/definitions/attributes/:attr_def_id/edit")]
    AttributeDefEditPage { attr_def_id: String },

    #[route("/admin/tags")]
    TagListPage {},

    #[route("/admin/tags/new")]
    TagNewPage {},

    #[route("/admin/tags/:id")]
    TagPage { id: Id },
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
            Route::AttributeDefEditPage { attr_def_id: _ } => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Attributes Definitions".into(), Route::AttributeDefListPage {}),
                ("Edit".into(), to),
            ],
            Route::TagListPage {} => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Tags".into(), to),
            ],
            Route::TagNewPage {} => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Tags".into(), Route::TagListPage {}),
                ("New".into(), to),
            ],
            Route::TagPage { id: _ } => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Tags".into(), Route::TagListPage {}),
                ("Edit".into(), to),
            ],
        }
    }
}
