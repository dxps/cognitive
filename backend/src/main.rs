use anyhow::Ok;

mod domain;
mod infra;

fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;

    infra::start_web_server();

    Ok(())
}
