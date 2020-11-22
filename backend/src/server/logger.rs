extern crate log;

use flexi_logger::{colored_detailed_format, Logger};
use log::info;

use crate::config::ServerConfig;

pub fn init_logger(config: &ServerConfig) {
    let log_level = config.log_level.to_string();
    Logger::with_env_or_str(&log_level)
        .format(colored_detailed_format)
        .start()
        .ok();

    info!("Initialized logger with level {}", &log_level);
}
