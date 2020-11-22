extern crate envconfig;

use envconfig::Envconfig;
use flexi_logger::Level;

#[derive(Envconfig)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_PORT", default = "3333")]
    pub port: u16,

    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: Level,
}
