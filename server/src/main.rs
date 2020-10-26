use anyhow::Result;
use server::Server;

fn main() -> Result<()> {
    let db_url = std::env::var("DATABASE_URL").or(Err(
        "DATABASE_URL env var must be set", // TODO:
    ))?;
    let serv = Server::new(db_url.as_str());
    serv.run();
    Ok(())
}
