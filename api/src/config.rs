use serde::Deserialize;

/// Application configuration loaded from environment variables.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    /// Parse configuration from environment variables.
    /// Expects `DATABASE_URL`, `HOST`, and `PORT` to be set.
    /// Falls back to defaults for any unset variable.
    pub fn from_env() -> Result<Self, String> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:./data/ulysses.db?mode=rwc".into());
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".into())
            .parse()
            .map_err(|e| format!("PORT must be a valid u16: {}", e))?;

        Ok(Self {
            database_url,
            host,
            port,
        })
    }
}
