use crate::config::AppConfig;
use flexi_logger::{colored_detailed_format, Logger};
use log::info;

pub fn init_logger(config: &AppConfig) {
    let log_level = config.log_level.to_string();
    Logger::with_env_or_str(&log_level)
        .format(colored_detailed_format)
        .start()
        .ok();

    info!("Initialized logger with level {}", &log_level);
}
