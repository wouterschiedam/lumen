use std::process;
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

    pub async fn explain(&self, sha: String) -> Result<(), LumenError> {
        let mut spinner = Spinner::new(spinners::Dots, "Loading...", Color::Blue);
        let commit = GitCommit::new(sha)?;
        let result = self.provider.explain(commit.clone()).await?;

        let result = format!(
            "commit {} | Author: {} <{}> | Date: {}\n\n{}\n-----\n{}",
            commit.full_hash,
            commit.author_name,
            commit.author_email,
            commit.date,
            commit.message,
            result
        );

        spinner.success("Done");

        // Try to use mdcat, fall back to direct printing if it fails
        match std::process::Command::new("mdcat")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(mut mdcat) => {
                if let Some(stdin) = mdcat.stdin.take() {
                    std::process::Command::new("echo")
                        .arg(&result)
                        .stdout(stdin)
                        .spawn()?
                        .wait()?;
                }
                let output = mdcat.wait_with_output()?;
                println!("{}", String::from_utf8(output.stdout)?);
            }
            Err(_) => {
                println!("{}", result);
            }
        }

        Ok(())
    }

    pub async fn list(&self) -> Result<(), LumenError> {
        let command = "git log --color=always --format='%C(auto)%h%d %s %C(black)%C(bold)%cr' | fzf --ansi --reverse --bind='enter:become(echo {1})' --wrap";

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
