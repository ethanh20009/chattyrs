use std::time::Duration;

use super::{
    error::{Error, Result},
    model::{AssistantMessage, LlmChat},
};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::environment::Environment;

pub struct LlmEngine {
    base_url: String,
    model: String,
    http_client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct LlmCompletionResponse {
    model: String,
    created_at: String,
    response: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LlmChatResponse {
    model: String,
    created_at: String,
    message: AssistantMessage,
}

impl LlmEngine {
    pub fn new(environment: &Environment) -> Result<LlmEngine> {
        Ok(LlmEngine {
            model: environment
                .llm
                .model
                .clone()
                .unwrap_or("llama3".to_string()),
            base_url: environment
                .llm
                .base_url
                .clone()
                .unwrap_or("http://localhost:11434/api/generate".to_string()),
            http_client: ClientBuilder::default()
                .timeout(Duration::from_secs(60))
                .build()?,
        })
    }

    pub async fn get_completion(&self, question: &str) -> Result<String> {
        let payload = json!({
            "model": self.model,
            "prompt": question,
            "stream": false,
            "options": {
                "seed": 123,
                "top_k": 20,
                "top_p": 0.9,
                "temperature": 0
            }
        });
        self.http_client
            .post(self.base_url.clone() + "/generate")
            .json(&payload)
            .send()
            .await
            .map_err(|err| Error::HTTPRequestFailed(err.to_string()))?
            .json::<LlmCompletionResponse>()
            .await
            .map_err(|_| Error::HTTPResponseParseFailed)
            .map(|res| res.response)
    }

    pub async fn get_chat_completion(&self, messages: LlmChat) -> Result<String> {
        let payload = json!({
            "model": self.model,
            "messages": messages,
            "stream": false,
            "options": {
                "seed": 123,
                "top_k": 20,
                "top_p": 0.9,
                "temperature": 0
            }
        });
        println!("{}", payload);
        self.http_client
            .post(self.base_url.clone() + "/chat")
            .json(&payload)
            .send()
            .await
            .map_err(|err| Error::HTTPRequestFailed(err.to_string()))?
            .json::<LlmChatResponse>()
            .await
            .map_err(|_| Error::HTTPResponseParseFailed)
            .map(|res| res.message.content)
    }
}
