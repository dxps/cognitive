#[cfg(feature = "server")]
use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
pub fn server_start(app_fn: fn() -> Element) {
    //
    use axum::routing::*;
    use dioxus::prelude::*;
    use tracing::debug;

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        debug!("Starting up ...");

        //let state = ServerState();

        // Build our application web api router.
        let web_api_router = Router::new()
            // Server side render the application, serve static assets, and register the server functions.
            .serve_dioxus_application(ServeConfig::builder().build(), move || VirtualDom::new(app_fn))
            .await;
        //.layer(Extension(state));

        // Start it.
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, web_api_router.into_make_service()).await.unwrap();
    });
}
