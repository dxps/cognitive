// #[cfg(feature = "web")]
// #[cfg(not(feature = "server"))]
use cognitive::ui;

fn main() {
    #[cfg(feature = "server")]
    dotenvy::dotenv().ok();

    #[cfg(feature = "server")]
    cognitive::backend::init_logging();

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        //
        cognitive::backend::print_db();

        // Create a new axum router for our Dioxus app.
        let router = dioxus::server::router(ui::App);

        // Customize it however you want.

        // And then return it.
        Ok(router)
    });

    // The `launch` function is the main entry point for the dioxus based UI.
    // It takes a component and renders it with the platform feature you have enabled.
    #[cfg(not(feature = "server"))]
    dioxus::launch(ui::App);

    // TODO: Graceful shutdown (at least for the database pool).
}
