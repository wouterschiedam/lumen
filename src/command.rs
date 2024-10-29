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
        let diff = std::process::Command::new("git")
            .args([
                "diff-tree",
                "-p",
                "--binary",
                "--no-color",
                "--compact-summary",
                &sha,
            ])
            .output()
            .expect("failed to execute process");

        let commit_message = std::process::Command::new("git")
            .args(["log", "--format=%B", "-n", "1", &sha])
            .output()
            .expect("failed to execute process");

        let result = self
            .provider
            .explain(
                String::from_utf8(diff.stdout).unwrap(),
                String::from_utf8(commit_message.stdout).unwrap(),
            )
            .await?;

        Ok(result)
    }
}
