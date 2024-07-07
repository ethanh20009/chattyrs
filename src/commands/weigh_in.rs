use serenity::all::{CommandInteraction, CreateCommand};

use super::error::*;
use crate::{
    environment::Environment,
    llm::{
        self,
        engine::LlmEngine,
        model::{AssistantMessage, LlmMessage, SystemMessage, UserMessage},
    },
};

pub fn register_weigh_in(environment: &Environment) -> CreateCommand {
    CreateCommand::new("weigh-in").description(format!(
        "Ask {} to comment on recent messages",
        environment.bot_name
    ))
}
pub async fn run_weigh_in<'a>(
    command: &CommandInteraction,
    llm_engine: &'a LlmEngine,
    http_client: &serenity::http::Http,
    environment: &Environment,
) -> Result<String> {
    let channel_id = command.channel_id;
    let latest_messages = http_client
        .get_messages(
            channel_id,
            None,
            Some(
                environment
                    .memory
                    .max_message_count
                    .try_into()
                    .expect("Max memory count could not be parsed into u8"),
            ),
        )
        .await
        .map_err(Error::from)?;

    let mut llm_context: Vec<LlmMessage> = latest_messages
        .into_iter()
        .map(|message| {
            if message.author.bot {
                AssistantMessage {
                    content: message.content,
                }
                .into()
            } else {
                UserMessage {
                    content: message.content,
                }
                .into()
            }
        })
        .collect();

    llm_context.reverse();

    llm_context.push(SystemMessage {
        content: "Give your thoughts on previous messages. Be brutally honest, and make your response humourous.".to_string()
    }.into());

    Ok(llm_engine
        .get_chat_completion(llm_context)
        .await
        .map_err(Error::from)?)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to retrieve latest_messages, {0}")]
    GetChannelFailed(#[from] serenity::Error),
    #[error("failed to retrieve response from llm, {0}")]
    LlmError(#[from] llm::error::Error),
}
