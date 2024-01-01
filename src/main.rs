mod db;
mod logger;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

type StdError = Box<dyn std::error::Error>;

fn main() -> Result<(), StdError> {
    dotenv::dotenv()?;
    logger::init()?;
    Ok(())
}
