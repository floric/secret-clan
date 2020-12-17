use std::thread;

use crate::config::AppConfig;
use flexi_logger::{DeferredNow, Level, Logger, Record};
use log::info;

pub fn custom_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] {} [{:?}|{}] {}",
        now.now().format("%Y-%m-%d %H:%M:%S%.3f %:z"),
        record.level(),
        thread::current().name().unwrap_or("-"),
        record.module_path().unwrap_or("-"),
        &record.args()
    )
}

pub fn init_logger(config: &AppConfig) {
    let log_level = if cfg!(test) {
        Level::Warn.to_string()
    } else {
        config.log_level.to_string()
    };
    Logger::with_env_or_str(&log_level)
        .format(custom_format)
        .start()
        .ok();

    info!("Initialized logger with level {}", &log_level);
}
