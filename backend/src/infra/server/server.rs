use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::{SessionPgPool, SessionPgSessionStore};
use shlib::{AppError, AppResult, domain::model::Id};
use sqlx::PgPool;

use crate::{
    domain::logic::UserMgmt,
    infra::{
        AuthUserAccount, SESSION_NAME, SESSION_TABLE, ServerState, connect_to_pgdb,
        http_api::{self},
        init_auth_layer, init_session_layer,
    },
};

pub fn start_web_server() {
    //
    init_logging();
    log::info!("Starting the server ...");

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

            let state = ServerState::new(Arc::new(pg_pool.clone()));

            register_admin_user(&state.user_mgmt)
                .await
                .expect("Self registering admin user failed");

            let auth_layer = init_auth_layer(&pg_pool).await;
            let session_layer = init_session_layer(&pg_pool).await;

            let web_api_router = Router::new()
                .route("/auth/login", axum::routing::post(http_api::login))
                .route("/auth/logout", axum::routing::post(http_api::logout))
                .layer(auth_layer)
                .layer(session_layer)
                .with_state(state);

            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            log::info!("Listening on {}", addr);
            axum::serve(listener, web_api_router).await.unwrap();
        });
}

fn init_logging() {
    use log::LevelFilter::{Debug, Info, Warn};

    simple_logger::SimpleLogger::new()
        .with_module_level("server", Debug) // The logging level for this crate.
        .with_module_level("sqlx", Info)
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
            _ => Err(app_err),
        },
    }
}
