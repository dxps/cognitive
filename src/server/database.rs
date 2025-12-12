/////////////////////
// the server side //
/////////////////////

use dioxus::fullstack::Lazy;

#[cfg(feature = "server")]
use sqlx::PgPool;

#[cfg(feature = "server")]
use crate::shared::AppError;

#[cfg(feature = "server")]
pub async fn connect_to_pgdb() -> Result<PgPool, AppError> {
    //
    use sqlx::postgres::PgPoolOptions;

    let db_url = std::env::var("DATABASE_URL").map_err(|err| {
        log::error!(
            "Unknown DATABASE_URL environment variable. Reason: '{}'.",
            err
        );
        AppError::Err("Unknown DATABASE_URL environment variable".into())
    })?;

    log::info!("Connecting to database ...");
    let pool = PgPoolOptions::new()
        .min_connections(2)
        .max_connections(8)
        .connect(db_url.as_str())
        .await
        .map_err(|e| {
            log::error!("Failed to connect to database: '{}'.", e);
            AppError::Err("Failed to connect to database".into())
        })?;

    Ok(pool)
}

#[cfg(feature = "server")]
pub static DB: Lazy<PgPool> = Lazy::new(|| async {
    #[cfg(feature = "server")]
    {
        use log::info;

        let db_pool = connect_to_pgdb()
            .await
            .expect("Failed to connect to database");
        info!("Connected to the database.");
        dioxus::Ok(db_pool)
    }

    #[cfg(not(feature = "server"))]
    {
        panic!("Database is only available on the server feature");
    }
});
