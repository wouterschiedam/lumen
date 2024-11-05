use std::process::Stdio;

use crate::error::LumenError;
use crate::git_commit::GitCommit;
use crate::git_staged::GitStaged;
use crate::provider::AIProvider;
use crate::provider::LumenProvider;

use spinoff::{spinners, Color, Spinner};

#[derive(Clone)]
pub enum Git {
    Commit(GitCommit),
    Staged(GitStaged),
}

pub struct LumenCommand {
    provider: LumenProvider,
}

impl LumenCommand {
    pub fn new(provider: LumenProvider) -> Self {
        LumenCommand { provider }
    }

    pub fn print_with_mdcat(&self, content: String) -> Result<(), LumenError> {
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
                let printline = String::from_utf8(output.stdout)?;
                let printline: String = printline
                    .lines()
                    .skip(1) // Skip the first line
                    .collect::<Vec<&str>>() // Collect remaining lines
                    .join("\n");

                println!("{}", printline);
            }
            Err(_) => {
                println!("{}", content);
            }
        }
        Ok(())
    }

    pub async fn explain(&self, sha: Option<String>) -> Result<(), LumenError> {
        let git;

        if let Some(commit_sha) = sha {
            // Handle the case where a commit SHA is provided
            git = Git::Commit(GitCommit::new(commit_sha)?);

            let result = match &git {
                Git::Commit(commit) => format!(
                    "`commit {}` | {} <{}> | {}\n\n{}\n-----\n",
                    commit.full_hash,
                    commit.author_name,
                    commit.author_email,
                    commit.date,
                    commit.message,
                ),
                Git::Staged(_) => {
                    return Err(LumenError::UnknownError(
                        "Expected a commit, but found staged changes.".into(),
                    ));
                }
            };

            self.print_with_mdcat(result)?;
        } else {
            git = Git::Staged(GitStaged::new()?);
        }

        let mut spinner = Spinner::new(spinners::Dots, "Generating Summary...", Color::Blue);

        // Pass the GitCommit object to the provider’s explain function
        let result = self.provider.explain(git.clone()).await?;

        spinner.success("Done");

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
