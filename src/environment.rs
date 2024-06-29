use dotenv::dotenv;
use serde::Deserialize;

use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub discord_token: String,
}

pub fn get_environment() -> Result<Environment> {
    dotenv().ok();
    Ok(config::Config::builder()
        .add_source(config::Environment::default().separator("__"))
        .build()?
        .try_deserialize()?)
}
