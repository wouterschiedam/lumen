use crate::{command::Git, git_commit::GitCommit};

use super::AIProvider;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PhindRequest {
    additional_extension_context: String,
    allow_magic_buttons: bool,
    is_vscode_extension: bool,
    message_history: Vec<Message>,
    requested_model: String,
    user_input: String,
}

#[derive(Debug, Deserialize)]
struct PhindResponse {
    choices: Option<Vec<Choice>>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    delta: Delta,
}

#[derive(Debug, Deserialize)]
struct Delta {
    content: String,
}

pub struct PhindProvider {
    client: reqwest::Client,
    model: String,
}

impl PhindProvider {
    pub fn new(client: reqwest::Client, model: Option<String>) -> Self {
        PhindProvider {
            client,
            model: model.unwrap_or_else(|| "Phind-70B".to_string()),
        }
    }

    async fn create_request(
        &self,
        commit_message: Option<&str>,
        diff_content: &str,
    ) -> Result<PhindRequest, Box<dyn std::error::Error>> {
        // Prioritize the diff content for the summary prompt if available
        let user_input = if let Some(message) = commit_message {
            format!(
                    "Please analyze this git commit and provide a summary.\n\nCommit Message:\n{}\n\nDiff Content:\n{}",
                    message, diff_content
                )
        } else if !diff_content.is_empty() {
            format!(
            "Please analyze the following staged changes and provide a short, concise title and a detailed summary.\n\nDiff Content:\n{}",
            diff_content
        )
        } else {
            "No commit message or diff content available.".to_string()
        };

        Ok(PhindRequest {
            additional_extension_context: String::new(),
            allow_magic_buttons: true,
            is_vscode_extension: true,
            message_history: vec![Message {
                content: user_input.clone(),
                role: "user".to_string(),
            }],
            requested_model: self.model.clone(),
            user_input,
        })
    }

    fn create_headers() -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("User-Agent", HeaderValue::from_static(""));
        headers.insert("Accept", HeaderValue::from_static("*/*"));
        headers.insert("Accept-Encoding", HeaderValue::from_static("Identity"));
        Ok(headers)
    }

    async fn get_main_text(response: &str) -> Result<String, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = response.split('\n').collect();
        let mut full_text = String::new();

        for line in lines {
            if line.starts_with("data: ") {
                let obj = line.strip_prefix("data: ").unwrap_or("{}");
                if let Ok(response) = serde_json::from_str::<PhindResponse>(obj) {
                    if let Some(choices) = response.choices {
                        if !choices.is_empty() {
                            full_text.push_str(&choices[0].delta.content);
                        }
                    }
                }
            }
        }

        Ok(full_text)
    }
}

#[async_trait]
impl AIProvider for PhindProvider {
    async fn explain(&self, git: Git) -> Result<String, Box<dyn std::error::Error>> {
        let request = match git {
            Git::Commit(ref commit) => {
                self.create_request(Some(commit.message.as_str()), commit.diff.as_str())
                    .await?
            }
            Git::Staged(ref staged) => self.create_request(None, staged.diff.as_str()).await?,
        };

        let headers = Self::create_headers()?;

        let response = self
            .client
            .post("https://https.extension.phind.com/agent/")
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .text()
            .await?;

        let res = Self::get_main_text(&response).await?;
        Ok(res)
    }
}
