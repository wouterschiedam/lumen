pub struct GitCommit {
    pub sha: String,
    pub message: String,
    pub diff: String,
    //author: String,
    //date: String,
}

impl GitCommit {
    pub fn new(sha: String) -> Self {
        GitCommit {
            sha: sha.clone(),
            message: Self::get_message(&sha),
            diff: Self::get_diff(&sha),
        }
    }

    fn get_diff(sha: &str) -> String {
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

        String::from_utf8(diff.stdout).unwrap()
    }

    fn get_message(sha: &str) -> String {
        let commit_message = std::process::Command::new("git")
            .args(["log", "--format=%B", "-n", "1", &sha])
            .output()
            .expect("failed to execute process");

        String::from_utf8(commit_message.stdout).unwrap()
    }
}
