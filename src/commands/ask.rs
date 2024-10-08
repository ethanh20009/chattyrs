use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};

use crate::{
    environment::Environment,
    llm::{self, engine::LlmEngine},
};

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
pub async fn run_ask<'a>(
    options: &'a [ResolvedOption<'_>],
    llm_engine: &'a LlmEngine,
) -> Result<String> {
    let question_response = &options.first().ok_or(Error::MissingQuestion)?.value;
    match question_response {
        ResolvedValue::String(question) => Ok((llm_engine
            .get_completion(question)
            .await
            .map(|llm_response| format!("**Question**: *{question}*\n{llm_response}"))
            .map_err(Error::from)?)
        .to_string()),
        _ => Err(Error::MissingQuestion.into()),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Question unanswerable")]
    Unanswerable,
    #[error("Missing question")]
    MissingQuestion,
    #[error("Failed to get completion from Llm Engine, {0})")]
    LlmEngineCompletionFailed(#[from] llm::error::Error),
}
