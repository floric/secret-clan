use envconfig::Envconfig;
use flexi_logger::Level;

#[derive(Envconfig, Clone)]
pub struct AppConfig {
    #[envconfig(from = "SERVER_PORT", default = "3333")]
    pub port: u16,

    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: Level,

    #[envconfig(from = "AUTH_SECRET", default = "super-secret")]
    pub auth_secret: String,
}
