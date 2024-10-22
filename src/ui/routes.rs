use crate::{
    domain::model::Id,
    ui::pages::{
        AdminPage, AttributeDefListPage, AttributeDefNewPage, AttributeDefPage, EntityDefListPage, EntityDefNewPage,
        EntityDefPage, EntityListPage, EntityNewPage, EntityPage, Home, Login, LoginIsRequiredPage, Logout, TagListPage,
        TagNewPage, TagPage, UserProfilePage,
    },
};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/login-required")]
    LoginIsRequiredPage {},

    #[route("/logout")]
    Logout {},

    #[route("/users/:username")]
    UserProfilePage { username: String },

    #[route("/admin")]
    AdminPage {},

    #[route("/admin/definitions/attributes")]
    AttributeDefListPage {},

    #[route("/admin/definitions/attributes/new")]
    AttributeDefNewPage {},

    #[route("/admin/definitions/attributes/:attr_def_id")]
    AttributeDefPage { attr_def_id: Id },

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
            Route::Login {} => vec![("Login".into(), Route::Login {})],
            Route::Logout {} => vec![("Logout".into(), to)],
            Route::UserProfilePage { username: _ } => vec![("User Profile".into(), to)],
            Route::AdminPage {} => vec![("Admin".into(), to)],
            Route::AttributeDefListPage {} => vec![("Admin".into(), Route::AdminPage {}), ("Attributes Definitions".into(), to)],
            Route::AttributeDefNewPage {} => vec![
                ("Admin".into(), Route::AdminPage {}),
                ("Attributes Definitions".into(), Route::AttributeDefListPage {}),
                ("New".into(), to),
            ],
            Route::EntityDefListPage {} => vec![
                ("Admin".into(), Route::AdminPage {}),
                ("Entities Definitions".into(), Route::EntityDefListPage {}),
            ],
            Route::EntityDefNewPage {} => vec![
                ("Admin".into(), Route::AdminPage {}),
                ("Entities Definitions".into(), Route::EntityDefListPage {}),
                ("New".into(), to),
            ],
            Route::EntityListPage {} => vec![
                ("Admin".into(), Route::AdminPage {}),
                ("Entities".into(), Route::EntityListPage {}),
            ],
            Route::EntityNewPage {} => vec![
                ("Admin".into(), Route::AdminPage {}),
                ("Entities".into(), Route::EntityListPage {}),
                ("New".into(), to),
            ],
            Route::TagListPage {} => vec![("Admin".into(), Route::AdminPage {}), ("Tags".into(), to)],
            Route::TagNewPage {} => vec![
                ("Admin".into(), Route::AdminPage {}),
                ("Tags".into(), Route::TagListPage {}),
                ("New".into(), to),
            ],
            Route::TagPage { id } => {
                let to = Route::TagPage { id: id.clone() };
                let tag_name = format!("id:{}", id);
                Route::get_path_to_tag(to, tag_name)
            }
            _ => vec![("Admin".into(), Route::AdminPage {})],
        }
    }

    pub fn get_path_to_tag(to: Route, tag_name: String) -> Vec<(String, Route)> {
        vec![
            ("Admin".into(), Route::AdminPage {}),
            ("Tags".into(), Route::TagListPage {}),
            (tag_name, to),
        ]
    }

    pub fn get_path_to_attr_def(to: Route, attr_def_name: String) -> Vec<(String, Route)> {
        vec![
            ("Admin".into(), Route::AdminPage {}),
            ("Attributes Definitions".into(), Route::AttributeDefListPage {}),
            (attr_def_name, to),
        ]
    }

    pub fn get_path_to_ent_def(to: Route, ent_def_name: String) -> Vec<(String, Route)> {
        vec![
            ("Admin".into(), Route::AdminPage {}),
            ("Entities Definitions".into(), Route::EntityDefListPage {}),
            (ent_def_name, to),
        ]
    }

    pub fn get_path_to_ent(to: Route, ent_def_name: String) -> Vec<(String, Route)> {
        vec![
            ("Admin".into(), Route::AdminPage {}),
            ("Entities".into(), Route::EntityListPage {}),
            (ent_def_name, to),
        ]
    }
}
