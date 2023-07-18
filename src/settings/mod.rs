use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http: HttpSettings,
    pub websocket: WebsocketSettings,
}

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpSettings {
    pub api_key: String,
    pub candlestick_url: String,
    pub server_url: String,
}

#[derive(Debug, Deserialize)]
pub struct WebsocketSettings {
    pub server_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let default_config = Config::builder().add_source(File::with_name("config/default"));

        // Load
        let decorated_config = if abbreviation(run_mode) == "dev" {
            default_config.add_source(File::with_name("config/development"))
        } else {
            default_config
        };

        let config = decorated_config.build()?;
        config.try_deserialize()
    }
}

fn abbreviation(s: String) -> String {
    s.chars().take(3).collect::<String>().to_lowercase()
}
