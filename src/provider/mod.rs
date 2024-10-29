use async_trait::async_trait;
use openai::OpenAIProvider;
use phind::PhindProvider;

use crate::ProviderType;

pub mod openai;
pub mod phind;

#[async_trait]
pub trait AIProvider {
    async fn explain(
        &self,
        diff: String,
        commit_message: String,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

pub enum LumenProvider {
    OpenAI(Box<OpenAIProvider>),
    Phind(Box<PhindProvider>),
}

impl LumenProvider {
    pub fn new(
        client: reqwest::Client,
        provider_type: ProviderType,
        api_key: Option<String>,
    ) -> Self {
        match provider_type {
            ProviderType::OpenAI => {
                let api_key = api_key.expect(
                    "api_key will always be Some when provider is OpenAI due to required_if_eq",
                );
                LumenProvider::OpenAI(Box::new(OpenAIProvider::new(client, api_key)))
            }
            ProviderType::Phind => LumenProvider::Phind(Box::new(PhindProvider::new(client, None))),
        }
    }
}

#[async_trait]
impl AIProvider for LumenProvider {
    async fn explain(
        &self,
        diff: String,
        commit_message: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            LumenProvider::OpenAI(provider) => provider.explain(diff, commit_message).await,
            LumenProvider::Phind(provider) => provider.explain(diff, commit_message).await,
        }
    }
}
