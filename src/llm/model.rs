use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct UserMessage {
    pub content: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct AssistantMessage {
    pub content: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SystemMessage {
    pub content: String,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(tag = "role")]
pub enum LlmMessage {
    #[serde(rename = "user")]
    UserMessage(UserMessage),
    #[serde(rename = "assistant")]
    AssistantMessage(AssistantMessage),
    #[serde(rename = "system")]
    SystemMessage(SystemMessage),
}

impl From<AssistantMessage> for LlmMessage {
    fn from(value: AssistantMessage) -> Self {
        Self::AssistantMessage(value)
    }
}

impl From<UserMessage> for LlmMessage {
    fn from(value: UserMessage) -> Self {
        Self::UserMessage(value)
    }
}

impl From<SystemMessage> for LlmMessage {
    fn from(value: SystemMessage) -> Self {
        Self::SystemMessage(value)
    }
}

pub type LlmChat = Vec<LlmMessage>;
