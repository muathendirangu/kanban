mod logger;
mod models;
#[macro_use]
extern crate diesel;
mod schema;

type StdError = Box<dyn std::error::Error>;

fn main() -> Result<(), StdError> {
    dotenv::dotenv()?;
    logger::init()?;
    Ok(())
}
