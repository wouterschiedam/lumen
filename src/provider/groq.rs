use super::AIProvider;
use crate::{command::Git, git_commit::GitCommit};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

pub struct GroqProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<GroqChoice>,
}

#[derive(Deserialize)]
struct GroqChoice {
    message: GroqMessage,
}

#[derive(Deserialize)]
struct GroqMessage {
    content: String,
}

impl GroqProvider {
    pub fn new(client: reqwest::Client, api_key: String, model: Option<String>) -> Self {
        GroqProvider {
            client,
            api_key,
            model: model.unwrap_or_else(|| "mixtral-8x7b-32768".to_string()),
        }
    }
}

async fn get_completion_result(
    client: &reqwest::Client,
    api_key: &str,
    payload: serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let groq_response: GroqResponse = response.json().await?;
    Ok(groq_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .unwrap_or_default())
}

#[async_trait]
impl AIProvider for GroqProvider {
    async fn explain(&self, git: Git) -> Result<String, Box<dyn std::error::Error>> {
        let user_input = match git {
            Git::Commit(ref commit) => {
                format!(
                    "Please analyze this git commit and provide a summary.\n\nCommit Message:\n{}\n\nDiff Content:\n{}",
                    commit.message.as_str(),
                    commit.diff.as_str()
                )
            }
            Git::Staged(ref staged) => {
                if !staged.diff.is_empty() {
                    format!(
                        "Please analyze the following staged changes and provide a short, concise title and a detailed summary.\n\nDiff Content:\n{}",
                        staged.diff.as_str()
                    )
                } else {
                    "No commit message or diff content available.".to_string()
                }
            }
        };

        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": r#"You are a helpful assistant that analyzes git commits. \
                                 Provide a concise summary of the changes based on the commit message and diff content. \
                                 Focus on the impact and purpose of the changes."#
                },
                {
                    "role": "user",
                    "content": user_input,
                }
            ]
        });

        let res = get_completion_result(&self.client, &self.api_key, payload).await?;
        Ok(res)
    }
}
