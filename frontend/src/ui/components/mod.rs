//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod navbar;
pub use navbar::*;

mod navbar_menu;
pub use navbar_menu::*;

mod icons;
pub use icons::*;

mod modals;
pub use modals::*;

mod card;
pub use card::*;
