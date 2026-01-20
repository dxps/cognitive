use crate::ui::{
    components::Navbar,
    views::{
        AttributeTemplateView, DataMgmtView, HomeView, LinkView, LoginView, LogoutView, ObjectTemplateView, ObjectView, UserProfileView,
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

        #[route("/mgmt/data")]
        DataMgmtView {},

        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] 
        // with the `:` syntax. In this case, id will match any integer like `/templates/attributes/123`.
        #[route("/templates/attributes/:id")]
        AttributeTemplateView {id: Id},

        #[route("/objects/:id")]
        ObjectView {id: Id},

        #[route("/templates/objects/:id")]
        ObjectTemplateView {id: Id},

        #[route("/links/:id")]
        LinkView {id: Id},

}
