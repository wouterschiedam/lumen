use std::process;
use std::process::Stdio;

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
        let commit = GitCommit::new(sha.clone());
        let result = self.provider.explain(commit.clone()).await?;
        let result = format!(
            "commit {}\nAuthor: {} <{}>\nDate: {}\n\n{}\n-----\n{}",
            sha, commit.author_name, commit.author_email, commit.date, commit.message, result
        );

        Ok(result)
    }

    pub async fn list(&self) -> Result<String, Box<dyn std::error::Error>> {
        let command = "git log --color=always --format='%C(auto)%h%d %s %C(black)%C(bold)%cr' | fzf --ansi --no-sort --reverse --bind='enter:become(echo {1})' --wrap";

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Command failed with status: {:?}", output.status);
            process::exit(1);
        }

        let mut sha = String::from_utf8(output.stdout).unwrap();
        sha.pop(); // remove trailing newline from echo

        self.explain(sha).await
    }
}
