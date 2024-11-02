use std::process::Stdio;

use crate::error::LumenError;
use crate::git_commit::GitCommit;
use crate::provider::AIProvider;
use crate::provider::LumenProvider;

use spinoff::{spinners, Color, Spinner};

pub struct LumenCommand {
    provider: LumenProvider,
}

impl LumenCommand {
    pub fn new(provider: LumenProvider) -> Self {
        LumenCommand { provider }
    }

    pub fn print_with_mdcat(&self, content: String) -> Result<(), LumenError> {
        let non_interactive = std::env::var("NON_INTERACTIVE").is_ok();

        if non_interactive {
            // In non-interactive mode, strip ANSI codes and print directly
            let stripped_content = String::from_utf8(strip_ansi_escapes::strip(&content)?)?;
            println!("{}", stripped_content);
            return Ok(());
        }

        match std::process::Command::new("mdcat")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(mut mdcat) => {
                if let Some(stdin) = mdcat.stdin.take() {
                    std::process::Command::new("echo")
                        .arg(&content)
                        .stdout(stdin)
                        .spawn()?
                        .wait()?;
                }
                let output = mdcat.wait_with_output()?;
                println!("{}", String::from_utf8(output.stdout)?);
            }
            Err(_) => {
                println!("{}", content);
            }
        }
        Ok(())
    }

    pub async fn explain(&self, sha: Option<String>) -> Result<(), LumenError> {
        let commit;

        // Check for non-interactive mode by looking for an environment variable
        let non_interactive = std::env::var("NON_INTERACTIVE").is_ok();

        if let Some(commit_sha) = sha {
            // Handle the case where a commit SHA is provided
            commit = GitCommit::new(commit_sha)?;
        } else {
            // No SHA provided, so create a GitCommit with the diff as its message
            let output = std::process::Command::new("git")
                .arg("diff")
                .arg("--staged")
                .output()?;

            if !output.status.success() {
                return Err(LumenError::UnknownError(
                    "Failed to retrieve staged diff".into(),
                ));
            }

            // Convert diff output to a String for the commit message
            let diff_content = String::from_utf8_lossy(&output.stdout).to_string();

            // Create a dummy GitCommit object with the diff as its message
            commit = GitCommit {
                full_hash: "diff".to_string(),
                author_name: "Staged Changes".to_string(),
                author_email: "noreply@example.com".to_string(),
                date: "Now".to_string(),
                message: "Summary of staged changes".to_string(),
                diff: diff_content, // Store the diff content here
            };
        }

        let mut spinner = if !non_interactive {
            Some(spinoff::Spinner::new(
                spinners::Dots,
                "Generating Summary...",
                Color::Blue,
            ))
        } else {
            None
        };

        // Pass the GitCommit object to the provider’s explain function
        let result = self.provider.explain(commit.clone()).await?;

        if let Some(mut spinner) = spinner {
            spinner.success("Done");
        }

        // Print the summary result
        self.print_with_mdcat(result)?;

        Ok(())
    }

    pub async fn list(&self) -> Result<(), LumenError> {
        let command = "git log --color=always --format='%C(auto)%h%d %s %C(black)%C(bold)%cr' | fzf --ansi --reverse --bind='enter:become(echo {1})' --wrap";

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let mut stderr = String::from_utf8(output.stderr)?;
            stderr.pop();

            let hint = match &stderr {
                stderr if stderr.contains("fzf: command not found") => {
                    Some("`list` command requires fzf")
                }
                _ => None,
            };

            let hint = match hint {
                Some(hint) => format!("(hint: {})", hint),
                None => String::new(),
            };

            return Err(LumenError::UnknownError(
                format!("{} {}", stderr, hint).into(),
            ));
        }

        let mut sha = String::from_utf8(output.stdout)?;
        sha.pop(); // remove trailing newline from echo

        self.explain(Some(sha)).await
    }
}
