use super::AIProvider;
use crate::{command::Git, git_commit::GitCommit};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

pub struct ClaudeProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Deserialize)]
struct ClaudeContent {
    text: String,
}

impl ClaudeProvider {
    pub fn new(client: reqwest::Client, api_key: String, model: Option<String>) -> Self {
        ClaudeProvider {
            client,
            api_key,
            model: model.unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string()),
        }
    }
}

async fn get_completion_result(
    client: &reqwest::Client,
    api_key: &str,
    payload: serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let claude_response: ClaudeResponse = response.json().await?;
    Ok(claude_response
        .content
        .first()
        .map(|content| content.text.clone())
        .unwrap_or_default())
}

#[async_trait]
impl AIProvider for ClaudeProvider {
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
            "max_tokens": 4096,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant that analyzes git commits. \
                               Provide a concise summary of the changes based on the commit message and diff content. \
                               Focus on the impact and purpose of the changes."
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
