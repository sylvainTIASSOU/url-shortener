#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            database_url: require("DATABASE_URL")?,
            port: require("PORT")
                .unwrap_or_else(|_| "3030".to_string())
                .parse()
                .map_err(|_| "PORT must be a number".to_string())?,
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string())
                .to_lowercase(),
        })
    }
}

fn require(key: &str) -> Result<String, String> {
    std::env::var(key).map_err(|_| format!("environment variable {} not set", key))
}
