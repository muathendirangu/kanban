use std::env;
use std::fs;
use log::{debug, info, error, trace, warn};

pub fn init() -> Result<(), fern::InitError> {
    // get log level from env variable
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string());
    let log_level = log_level.parse::<log::LevelFilter>().unwrap_or(log::LevelFilter::Info);

    let mut log_builder = fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}][{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.target(),
            record.level(),
            message))
    })
    .level(log_level)
    .chain(std::io::stdout());// log to stderr

    // log to file if provided
    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = fs::File::create(log_file)?;
        log_builder = log_builder.chain(log_file);
    }

    log_builder.apply()?;
    trace!("TRACE output enabled");
    debug!("DEBUG output enabled");
    info!("INFO output enabled");
    warn!("WARN output enabled");
    error!("ERROR output enabled");
    Ok(())
}
