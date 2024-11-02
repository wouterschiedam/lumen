use std::{io, process::Command, string::FromUtf8Error};

#[derive(Debug, Clone)]
pub enum GitStagedError {
    CommandError(String),
    EmptyDiff(),
}

impl From<io::Error> for GitStagedError {
    fn from(err: io::Error) -> GitStagedError {
        GitStagedError::CommandError(err.to_string())
    }
}

impl From<FromUtf8Error> for GitStagedError {
    fn from(err: FromUtf8Error) -> GitStagedError {
        GitStagedError::CommandError(err.to_string())
    }
}

impl std::fmt::Display for GitStagedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitStagedError::CommandError(err) => write!(f, "{err}"),
            GitStagedError::EmptyDiff() => write!(f, "Diff for staged changes is empty"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GitStaged {
    pub diff: String,
}

impl GitStaged {
    pub fn new() -> Result<Self, GitStagedError> {
        Ok(GitStaged {
            diff: Self::get_staged_diff()?,
        })
    }

    fn get_staged_diff() -> Result<String, GitStagedError> {
        let output = Command::new("git")
            .args(["diff", "--staged"])
            .output()
            .expect("Failed to execute git command");

        let diff = String::from_utf8(output.stdout)?;

        if diff.is_empty() {
            return Err(GitStagedError::EmptyDiff());
        }

        Ok(diff)
    }
}
