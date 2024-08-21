use serde_json::error;
use serenity::all::{CommandInteraction, CreateCommand};

use super::error::*;
use crate::{
    environment::Environment,
    llm::{
        self,
        engine::LlmEngine,
        model::{LlmMessage, SystemMessage, UserMessage},
    },
    vec_db::db_handler::VdbHandler,
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
    vec_db_client: &'a VdbHandler,
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

    let user_messages: UserMessage = latest_messages
        .into_iter()
        .rev()
        .filter(|message| !message.author.bot)
        .fold(
            UserMessage {
                content: String::new(),
            },
            |mut acc, message| {
                let user_message = format!(
                    "({}) {} said: `{}`\n",
                    message.timestamp.format("%d/%m/%Y %H:%M"),
                    message.author.name,
                    message.content
                );
                acc.content.push_str(&user_message);
                acc
            },
        );
    let relevant_messages = find_near_messages(
        &user_messages.content,
        command
            .guild_id
            .ok_or(Error::MissingGuildID)?
            .get()
            .to_string()
            .as_str(),
        llm_engine,
        vec_db_client,
    )
    .await?;

    let compiled_user_messages = user_messages.into();

    let system_message = SystemMessage {
        content: environment.llm.system_prompt.to_string()
            + "\n"
            + generate_relevant_message_prompt(relevant_messages)
                .unwrap_or("".to_string())
                .as_str(),
    }
    .into();
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

async fn find_near_messages<'a>(
    message: &'a str,
    guild_id: &str,
    llm_engine: &'a LlmEngine,
    vec_db_client: &'a VdbHandler,
) -> Result<Vec<String>> {
    let message = message.to_string();
    let embedding = llm_engine.get_embed(message).await.map_err(Error::from)?;
    let close_messages = vec_db_client
        .get_close_vectors(embedding, guild_id)
        .await
        .map_err(Error::VectorDB)?;
    Ok(close_messages
        .into_iter()
        .map(|point| point.message)
        .collect())
}

fn generate_relevant_message_prompt(messages: Vec<String>) -> Option<String> {
    Some(format!( "Using RAG retrieval, the following messages may or may not contain relevant information of messages that were sent in the past.\nRETRIEVED_MESSAGES\n{}\nEND_OF_RETRIEVED_MESSAGES", messages.into_iter().reduce(|acc, msg| acc + "\n" + &msg)?))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to retrieve latest_messages, {0}")]
    GetChannelFailed(#[from] serenity::Error),
    #[error("failed to retrieve response from llm, {0}")]
    LlmError(#[from] llm::error::Error),
    #[error("Failed to retrieve response from vector database client.\n{0}")]
    VectorDB(anyhow::Error),
    #[error("Command missing guild_id. It's likely the command was run from within dms.")]
    MissingGuildID,
}
