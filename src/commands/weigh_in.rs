use serenity::all::{CommandInteraction, CreateCommand};

use super::error::*;
use crate::{
    environment::Environment,
    llm::{
        self,
        engine::LlmEngine,
        model::{LlmMessage, SystemMessage, UserMessage},
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

    let compiled_user_messages: LlmMessage = latest_messages
        .into_iter()
        .rev()
        .filter(|message| !message.author.bot)
        .fold(
            UserMessage {
                content: String::new(),
            },
            |mut acc, message| {
                let user_message = format!("{} said: `{}`\n", message.author.name, message.content);
                acc.content.push_str(&user_message);
                acc
            },
        )
        .into();

    let system_message = SystemMessage {
        content: "Your purpose is to send a message responding to the other users. Give your own opinion on the matter, take a certain stance. Make your response humourous. Never respond with an empty reply. Keep your responses length to around a paragraph or a couple of sentences. If a longer answer is strictly judged as needed, break it up with two newlines per paragraph. Pay more attention to the messages at the end of the conversation.".to_string()
    }.into();
    let llm_context = vec![system_message, compiled_user_messages];

    Ok(llm_engine
        .get_chat_completion(llm_context)
        .await
        .and_then(|str_response| {
            if str_response.chars().count().lt(&1_usize) {
                Err(llm::error::Error::EmptyResponseError)
            } else {
                Ok(str_response)
            }
        })
        .map_err(Error::from)?)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to retrieve latest_messages, {0}")]
    GetChannelFailed(#[from] serenity::Error),
    #[error("failed to retrieve response from llm, {0}")]
    LlmError(#[from] llm::error::Error),
}
