#[cfg(feature = "server")]
pub fn init_logging() {
    use log::LevelFilter::{Info, Warn};

    simple_logger::SimpleLogger::new()
        .with_module_level("sqlx", Info)
        .with_module_level("tungstenite", Warn)
        .with_module_level("tungstenite::protocol", Warn)
        .with_module_level("tokio_tungstenite", Warn)
        // .with_module_level("axum_session", Info)
        // .with_module_level("axum_session_auth", Warn)
        // .with_module_level("dioxus_core", Warn)
        // .with_module_level("dioxus_signals", Warn)
        .with_module_level("warnings", Warn)
        .with_module_level("tracing", Warn)
        .init()
        .unwrap();
}
