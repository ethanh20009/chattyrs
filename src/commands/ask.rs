use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption};

use crate::environment::Environment;

use super::error::Result;

pub fn register_ask(environment: &Environment) -> CreateCommand {
    CreateCommand::new("ask")
        .description(format!("Ask {} a question", environment.bot_name))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "question",
                format!(
                    "The question you would like to ask {}",
                    environment.bot_name
                ),
            )
            .max_length(300)
            .required(true),
        )
}
pub fn run_ask(_options: &[ResolvedOption]) -> Result<String> {
    Ok("Hello".to_string())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Question unanswerable")]
    Unanswerable,
}
