#[cfg(feature = "server")]
use dioxus::dioxus_core::Element;

#[cfg(feature = "server")]
pub fn start_web_server(app_fn: fn() -> Element) {
    //
    use crate::{
        domain::model::UserAccount,
        server::{connect_to_pgdb, ws_handler, ServerState},
    };
    use axum::{routing::*, Extension};
    use axum_session::{SessionConfig, SessionLayer};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use axum_session_sqlx::{SessionPgPool, SessionPgSessionStore};
    use dioxus::prelude::*;
    use sqlx::PgPool;
    use std::{net::SocketAddr, sync::Arc};

    init_logging();
    log::info!("Starting up the server ...");

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        //
        log::info!("Connecting to the database ...");
        let pg_pool = connect_to_pgdb().await;
        if pg_pool.is_err() {
            log::error!(
                "Failed to connect to database due to '{}'. Exiting now!",
                pg_pool.unwrap_err()
            );
            return;
        }
        let pg_pool = pg_pool.unwrap();
        log::info!("Connected to the database.");

        // This defaults as normal cookies.
        let session_config = SessionConfig::default().with_table_name("user_sessions");
        let auth_config = AuthConfig::<String>::default().with_anonymous_user_id(Some("iH26rJ8Cp".to_string()));
        let session_store = SessionPgSessionStore::new(Some(pg_pool.clone().into()), session_config)
            .await
            .unwrap();

        let state = ServerState::new(Arc::new(pg_pool.clone()));

        state
            .auth_mgr
            .register_admin_user("admin@localhost".into(), "admin".into(), "admin".into())
            .await
            .expect("Self registering admin user failed");

        let web_api_router = Router::new()
            // Server side render the application, serve static assets, and register server functions.
            .serve_dioxus_application(ServeConfig::builder().build(), move || VirtualDom::new(app_fn))
            .await
            .layer(AuthSessionLayer::<UserAccount, String, SessionPgPool, PgPool>::new(Some(pg_pool)).with_config(auth_config))
            .layer(SessionLayer::new(session_store))
            .layer(Extension(state));

        let ws_router = Router::new().route("/", get(ws_handler));
        // .serve_dioxus_application(ServeConfig::builder().build(), move || VirtualDom::new(app_fn))
        // .await;

        let router = web_api_router.nest("/ws", ws_router);

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
            // axum::serve(listener, web_api_router.into_make_service_with_connect_info::<SocketAddr>())
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
