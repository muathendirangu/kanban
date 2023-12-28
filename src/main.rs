mod logger;
type StdError = Box<dyn std::error::Error>;

fn main() -> Result<(), StdError> {
    dotenv::dotenv()?;
    logger::init()?;
    Ok(())
}
