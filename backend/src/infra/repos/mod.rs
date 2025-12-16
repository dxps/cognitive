mod app_err_gen;
pub use app_err_gen::*;

mod user_repo;
pub use user_repo::*;

use shlib::AppError;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_to_pgdb() -> Result<PgPool, AppError> {
    //
    let db_url = std::env::var("DATABASE_URL").map_err(|err| {
        log::error!(
            "Unknown DATABASE_URL environment variable. Reason: '{}'.",
            err
        );
        AppError::Err("Unknown DATABASE_URL environment variable".into())
    })?;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(db_url.as_str())
        .await
        .map_err(|_| AppError::Err("Failed to connect to database".into()))?;
    Ok(pool)
}
