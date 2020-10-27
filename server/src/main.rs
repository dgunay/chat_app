use anyhow::{Context, Result};
use diesel::prelude::*;
use server::Server;

fn main() -> Result<()> {
    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL needs to be set")?;
    let serv = Server::new(server::store::Store::new(diesel::PgConnection::establish(
        db_url.as_str(),
    )?));
    serv.run();
    Ok(())
}
