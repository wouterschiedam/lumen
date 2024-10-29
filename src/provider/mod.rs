use async_trait::async_trait;

pub mod openai;
pub mod phind;

#[async_trait]
pub trait LumenProvider {
    async fn explain(
        &self,
        diff: String,
        commit_message: String,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
