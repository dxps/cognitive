use crate::ui::views::Navbar;
use crate::ui::views::{Blog, Home};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip] 
pub enum Route {

    #[layout(Navbar)]

        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] 
        // with the `:` syntax. In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]

        // Fields of the route variant will be passed to the component as props.
        // In this case, the blog component must accept an `id` prop of type `i32`.
        Blog { id: i32 },
}
