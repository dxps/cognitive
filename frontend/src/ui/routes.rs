use crate::ui::{
    components::Navbar,
    views::{
        AdminView, AttributeTemplateView, Blog, HomeView, LinkView, LoginView, LogoutView, ObjectTemplateView, ObjectView, UserProfileView,
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
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] 
        // with the `:` syntax. In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]

        // Fields of the route variant will be passed to the component as props.
        // In this case, the blog component must accept an `id` prop of type `i32`.
        Blog { id: i32 },

        #[route("/login")]
        LoginView {},

        #[route("/logout")]
        LogoutView {},

        #[route("/user-profile")]
        UserProfileView {},

        #[route("/admin")]
        AdminView {},

        #[route("/objects/:id")]
        ObjectView {id: Id},

        #[route("/templates/objects/:id")]
        ObjectTemplateView {id: Id},

        #[route("/templates/attributes/:id")]
        AttributeTemplateView {id: Id},

        #[route("/links/:id")]
        LinkView {id: Id},
}
