//! The views module contains the components for all Layouts and Routes for our app.
//! Each layout and route in our [`Route`] enum will render one of these components.
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout.
//! The layout defines a common wrapper around all child routes.

mod home_view;
pub use home_view::HomeView;

mod blog_view;
pub use blog_view::Blog;

mod admin_view;
pub use admin_view::*;

mod login_view;
pub use login_view::*;

mod logout_view;
pub use logout_view::*;

mod user_profile_view;
pub use user_profile_view::*;
