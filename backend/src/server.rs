use crate::database::connect_to_pgdb;

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
        });
}

fn init_logging() {
    use log::LevelFilter::{Info, Warn};

    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", Info)
        .with_module_level("tungstenite", Info)
        .with_module_level("tokio_tungstenite", Info)
        .with_module_level("axum_session", Info)
        .with_module_level("axum_session_auth", Warn)
        .with_module_level("dioxus_core", Warn)
        .with_module_level("dioxus_signals", Warn)
        .with_module_level("warnings", Warn)
        .with_module_level("tracing", Warn)
        .init()
        .unwrap();
}
