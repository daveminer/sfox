use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub auth: AuthSettings,
    pub http: HttpSettings,
    pub websocket: WebsocketSettings,
}

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpSettings {
    pub candlestick_url: String,
    pub server_url: String,
}

#[derive(Debug, Deserialize)]
pub struct WebsocketSettings {
    pub server_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let staging_env = env::var("SFOX_API_USE_STAGING").unwrap_or_else(|_| "false".to_string());
        let staging_api_enabled = staging_env == "true";

        let default_config =
            Config::builder().add_source(File::with_name("src/settings/config/default"));

        // Load
        let decorated_config = if staging_api_enabled {
            println!("Loading development configuration.");
            default_config.add_source(File::with_name("src/settings/config/development"))
        } else {
            default_config
        };

        let config = decorated_config.build()?;
        config.try_deserialize()
    }
}
