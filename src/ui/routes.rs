use crate::ui::pages::{AdminArea, Home, Login, Logout, UserProfile};
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
    AdminArea {},
}
