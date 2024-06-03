use dotenv::dotenv;
use std::env;

pub struct Config {
    pub binance_api_key: String,
    pub openai_api_key: String,
}

impl Config {
    pub fn load() -> Result<Self, std::env::VarError> {
        dotenv().ok();

        Ok(Config {
            binance_api_key: env::var("BINANCE_API_KEY")?,
            openai_api_key: env::var("OPENAI_API_KEY")?,
        })
    }
}

fn get_env(key: &str, default_value: &str) -> Result<String, std::env::VarError> {
    match env::var(key) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Ok(default_value.to_string()),
        Err(e) => Err(e),
    }
}
