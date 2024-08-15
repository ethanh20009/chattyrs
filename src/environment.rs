use dotenv::dotenv;
use serde::Deserialize;

use super::error::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Environment {
    pub discord_token: String,
    pub bot_name: String,
    pub llm: LlmOptions,
    pub memory: MemoryOptions,
    pub vdb: VectorDBOptions,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LlmOptions {
    pub model: Option<String>,
    pub base_url: Option<String>,
    pub system_prompt: String,
    pub embed_model: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VectorDBOptions {
    pub base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MemoryOptions {
    pub max_message_count: usize,
}

pub fn get_environment() -> Result<Environment> {
    dotenv().ok();
    Ok(config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::Environment::default().separator("__"))
        .build()?
        .try_deserialize()?)
}
