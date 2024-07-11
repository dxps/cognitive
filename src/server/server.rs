#[cfg(feature = "server")]
use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
pub fn start_web_server(app_fn: fn() -> Element) {
    //
    use axum::routing::*;
    use dioxus::prelude::*;
    use std::net::SocketAddr;
    use tracing::debug;

    use crate::server::ws_handler;

    init_logging();
    log::info!("Starting up the server ...");

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        debug!("Starting up ...");

        //let state = ServerState();

        // Build our application web api router.
        let app = Router::new()
            .route("/ws", get(ws_handler))
            // Server side render the application, serve static assets, and register the server functions.
            .serve_dioxus_application(ServeConfig::builder().build(), move || VirtualDom::new(app_fn))
            .await;
        //.layer(Extension(state));

        // Start it.
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    });
}

#[cfg(feature = "server")]
fn init_logging() {
    use log::LevelFilter::{Info, Warn};

    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", Info)
        .with_module_level("tungstenite", Info)
        .with_module_level("tokio_tungstenite", Info)
        .with_module_level("axum_session", Info)
        .with_module_level("axum_session_auth", Warn)
        .with_module_level("dioxus_core", Warn)
        .with_module_level("dioxus_signals", Info)
        .with_module_level("tracing", Warn)
        .init()
        .unwrap();
}
