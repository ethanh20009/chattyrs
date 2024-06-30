use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};

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
pub fn run_ask(options: &[ResolvedOption]) -> Result<String> {
    let question_response = &options.first().ok_or(Error::MissingQuestion)?.value;
    match question_response {
        ResolvedValue::String(question) => Ok(format!(
            "Question: {}\n\n{}",
            question, "I don't think anything yet"
        )),
        _ => Err(Error::MissingQuestion.into()),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Question unanswerable")]
    Unanswerable,
    #[error("Missing question")]
    MissingQuestion,
}
