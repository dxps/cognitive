use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use http::{HeaderValue, Method};
use shlib::{AppError, AppResult};
use tokio::signal;
use tower_http::cors::CorsLayer;

use crate::{
    domain::logic::UserMgmt,
    infra::{
        ServerState, connect_to_pgdb, disconnect_from_pgdb,
        http_api::{self},
        init_auth_layer, init_session_layer,
    },
};

pub fn start_web_server() {
    //
    init_logging();
    log::info!("Starting up ...");

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
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

            let http_port: u16 = std::env::var("HTTP_PORT")
                .expect("HTTP_PORT is not set")
                .parse()
                .expect(
                    format!(
                        "HTTP_PORT (with value '{}') is not a number",
                        std::env::var("HTTP_PORT").unwrap()
                    )
                    .as_str(),
                );

            let state = ServerState::new(Arc::new(pg_pool.clone()));

            register_admin_user(&state.user_mgmt)
                .await
                .expect("Self registering admin user failed");

            let auth_layer = init_auth_layer(&pg_pool).await;
            let session_layer = init_session_layer(&pg_pool).await;
            let cors_layer = CorsLayer::new()
                // set this to your actual frontend origin (Dioxus dev server, etc.)
                .allow_origin(HeaderValue::from_static("http://localhost:8080"))
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_headers([http::header::CONTENT_TYPE]);

            let web_api_router = Router::new()
                .route("/auth/login", axum::routing::post(http_api::login))
                .route("/auth/logout", axum::routing::post(http_api::logout))
                .layer(auth_layer)
                .layer(session_layer)
                .layer(cors_layer)
                .with_state(state);

            let addr = SocketAddr::from(([127, 0, 0, 1], http_port));
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            log::info!("Listening on {}", addr);
            axum::serve(listener, web_api_router)
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
            log::info!("Shutdown complete.");
            disconnect_from_pgdb(pg_pool).await;
        });
}

fn init_logging() {
    use log::LevelFilter::{Debug, Info, Warn};

    simple_logger::SimpleLogger::new()
        .with_module_level("server", Debug) // The logging level for this crate.
        .with_module_level("sqlx", Info)
        .with_module_level("sqlx::postgres", Warn)
        .with_module_level("tungstenite", Info)
        .with_module_level("tokio_tungstenite", Info)
        .with_module_level("axum", Info)
        .with_module_level("axum_session", Info)
        .with_module_level("axum_session_auth", Warn)
        .with_module_level("dioxus_core", Warn)
        .with_module_level("dioxus_signals", Warn)
        .with_module_level("warnings", Warn)
        .with_module_level("tracing", Warn)
        .init()
        .unwrap();
}

async fn register_admin_user(user_mgmt: &UserMgmt) -> AppResult<()> {
    //
    let email = "admin@local".to_string();
    let username = "admin".to_string();
    let password = "admin".to_string();
    match user_mgmt
        .register_admin_user(&email, &username, password)
        .await
    {
        Ok(id) => {
            log::debug!("Registered admin user w/ email: {}, id: {}", email, id);
            Ok(())
        }
        Err(app_err) => match app_err {
            AppError::AlreadyExists(_) => {
                log::debug!("Admin user is already registered.");
                Ok(())
            }
            _ => {
                log::debug!("Admin user registration failed: {:#?}", app_err);
                Err(app_err)
            }
        },
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { log::info!("Ctrl-C signal received. Starting graceful shutdown ..."); },
        _ = terminate => { log::info!("Termination signal received. Starting graceful shutdown ..."); },
    }
}
