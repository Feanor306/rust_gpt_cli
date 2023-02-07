use log::{info, error};
use flexi_logger::{Logger, LoggerHandle, FileSpec, WriteMode};

pub fn init_logger() -> Result<LoggerHandle, Box<dyn std::error::Error>> {
    let logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().directory("log"))
        .write_mode(WriteMode::Async)
        .start();
    let lh = match logger {
        Ok(l) => l,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
    return Ok(lh)
}

pub fn log_info(t: &str) {
    info!("{}", t);
}

pub fn log_err(t: &str) {
    error!("{}", t);
}
