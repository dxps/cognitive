use anyhow::Ok;

mod database;
mod server;

fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;

    server::start_web_server();

    Ok(())
}
