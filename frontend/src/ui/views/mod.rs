//! The views module contains the components for all Layouts and Routes for our app.
//! Each layout and route in our [`Route`] enum will render one of these components.
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout.
//! The layout defines a common wrapper around all child routes.

mod home_view;
pub use home_view::HomeView;

mod data_view;
pub use data_view::*;

mod login_view;
pub use login_view::*;

mod logout_view;
pub use logout_view::*;

mod user_profile_view;

pub use user_profile_view::*;

mod obj_view;
pub use obj_view::*;

mod obj_tmpl_view;
pub use obj_tmpl_view::*;

mod attr_tmpls_list_view;
pub use attr_tmpls_list_view::*;

mod attr_tmpl_view;
pub use attr_tmpl_view::*;

mod link_view;
pub use link_view::*;
