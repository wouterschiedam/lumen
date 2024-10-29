use crate::git_commit::GitCommit;

use super::AIProvider;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

pub struct OpenAIProvider {
    client: reqwest::Client,
    api_key: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

impl OpenAIProvider {
    pub fn new(client: reqwest::Client, api_key: String) -> Self {
        OpenAIProvider { client, api_key }
    }
}

async fn get_completion_result(
    client: &reqwest::Client,
    api_key: &str,
    payload: serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let openai_response: OpenAIResponse = response.json().await?;
    Ok(openai_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .unwrap_or_default())
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn explain(&self, commit: GitCommit) -> Result<String, Box<dyn std::error::Error>> {
        let payload = json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "system",
                    "content": r#"You are a helpful assistant that analyzes git commits. \
                                 Provide a concise summary of the changes based on the commit message and diff content. \
                                 Focus on the impact and purpose of the changes."#
                },
                {
                    "role": "user",
                    "content": format!(
                        "Diff:\n{}\n\nCommit message:\n{}\n\nPlease provide a clear and concise summary of these changes. Don't sound like an AI. Don't use filler words.",
                        commit.diff,
                        commit.message
                    ),
                }
            ]
        });

        let res = get_completion_result(&self.client, &self.api_key, payload).await?;
        Ok(res)
    }
}
