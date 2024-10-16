use crate::server::AppError;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[cfg(feature = "server")]
pub async fn connect_to_pgdb() -> Result<PgPool, AppError> {
    //

    use super::AppError;
    let db_url = std::env::var("DATABASE_URL").map_err(|_| AppError::Err("Unknown DATABASE_URL".into()))?;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(db_url.as_str())
        .await
        .map_err(|_| AppError::Err("Failed to connect to database".into()))?;
    Ok(pool)
}
