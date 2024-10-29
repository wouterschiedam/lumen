use crate::git_commit::GitCommit;
use crate::provider::AIProvider;
use crate::provider::LumenProvider;

pub struct LumenCommand {
    provider: LumenProvider,
}

impl LumenCommand {
    pub fn new(provider: LumenProvider) -> Self {
        LumenCommand { provider }
    }

    pub async fn explain(&self, sha: String) -> Result<String, Box<dyn std::error::Error>> {
        let commit = GitCommit::new(sha);
        dbg!(&commit);

        let result = self.provider.explain(commit).await?;

        Ok(result)
    }
}
