use crate::{
    domain::model::Id,
    ui::pages::{
        Admin, AttributeDefListPage, AttributeDefNewPage, AttributeDefPage, EntityDefListPage, EntityDefNewPage, EntityDefPage,
        EntityListPage, EntityNewPage, EntityPage, Home, Login, Logout, TagListPage, TagNewPage, TagPage, UserProfile,
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

    #[route("/admin/definitions/attributes/:attr_def_id")]
    AttributeDefPage { attr_def_id: String },

    #[route("/admin/definitions/entities")]
    EntityDefListPage {},

    #[route("/admin/definitions/entities/new")]
    EntityDefNewPage {},

    #[route("/admin/definitions/entities/:id")]
    EntityDefPage { id: Id },

    #[route("/admin/entities")]
    EntityListPage {},

    #[route("/admin/entities/new")]
    EntityNewPage {},

    #[route("/admin/entities/:id")]
    EntityPage { id: Id },

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
            Route::EntityDefListPage {} => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Entities Definitions".into(), Route::EntityDefListPage {}),
            ],
            Route::EntityDefNewPage {} => vec![
                ("Home".into(), Route::Home {}),
                ("Admin".into(), Route::Admin {}),
                ("Entities Definitions".into(), Route::EntityDefListPage {}),
                ("New".into(), to),
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
            Route::TagPage { id } => {
                let to = Route::TagPage { id: id.clone() };
                let tag_name = format!("id:{}", id);
                Route::get_path_to_tag(to, tag_name)
            }
            _ => vec![("Home".into(), Route::Home {}), ("Admin".into(), Route::Admin {})],
        }
    }

    pub fn get_path_to_tag(to: Route, tag_name: String) -> Vec<(String, Route)> {
        vec![
            ("Home".into(), Route::Home {}),
            ("Admin".into(), Route::Admin {}),
            ("Tags".into(), Route::TagListPage {}),
            (tag_name, to),
        ]
    }

    pub fn get_path_to_attr_def(to: Route, attr_def_name: String) -> Vec<(String, Route)> {
        vec![
            ("Home".into(), Route::Home {}),
            ("Admin".into(), Route::Admin {}),
            ("Attributes Definitions".into(), Route::AttributeDefListPage {}),
            (attr_def_name, to),
        ]
    }

    pub fn get_path_to_ent_def(to: Route, ent_def_name: String) -> Vec<(String, Route)> {
        vec![
            ("Home".into(), Route::Home {}),
            ("Admin".into(), Route::Admin {}),
            ("Entities Definitions".into(), Route::EntityDefListPage {}),
            (ent_def_name, to),
        ]
    }
}
