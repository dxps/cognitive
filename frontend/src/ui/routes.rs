use crate::ui::{
    components::Navbar,
    views::{
        AttributeTemplateView, AttributeTemplatesListView, DataView, HomeView, LinkView, LoginView, LogoutView, ObjectTemplateView,
        ObjectView, UserProfileView,
    },
};
use dioxus::prelude::*;
use shlib::domain::model::Id;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip] 
pub enum Route {

    #[layout(Navbar)]

        #[route("/")]
        HomeView {},

        #[route("/login")]
        LoginView {},

        #[route("/logout")]
        LogoutView {},

        #[route("/user-profile")]
        UserProfileView {},

        #[route("/data")]
        DataView {},

        #[route("/data/templates/attributes")]
        AttributeTemplatesListView {},

        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] 
        // with the `:` syntax. In this case, id will match any integer like `/templates/attributes/123`.
        #[route("/data/templates/attributes/:id")]
        AttributeTemplateView {id: Id},

        #[route("/data/objects/:id")]
        ObjectView {id: Id},

        #[route("/data/templates/objects/:id")]
        ObjectTemplateView {id: Id},

        #[route("/data/links/:id")]
        LinkView {id: Id},

}
