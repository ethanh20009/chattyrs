use dotenv::dotenv;
use serde::Deserialize;

use super::error::Result;

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub discord_token: String,
    pub bot_name: String,
    pub llm: LlmOptions,
}

#[derive(Debug, Deserialize)]
pub struct LlmOptions {
    pub model: Option<String>,
    pub base_url: Option<String>,
}

pub fn get_environment() -> Result<Environment> {
    dotenv().ok();
    Ok(config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::Environment::default().separator("__"))
        .build()?
        .try_deserialize()?)
}
