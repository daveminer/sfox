struct SFox {
    httpUrl: String,
    wsUrl: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub sfox: SFox,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    }
}
