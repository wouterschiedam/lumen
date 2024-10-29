#[derive(Clone, Debug)]
pub struct GitCommit {
    pub sha: String,
    pub message: String,
    pub diff: String,
    pub author_name: String,
    pub author_email: String,
    pub date: String,
}

impl GitCommit {
    pub fn new(sha: String) -> Self {
        GitCommit {
            sha: sha.clone(),
            message: Self::get_message(&sha),
            diff: Self::get_diff(&sha),
            author_name: Self::get_author_name(&sha),
            author_email: Self::get_author_email(&sha),
            date: Self::get_date(&sha),
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

    fn get_author_name(sha: &str) -> String {
        let commit_message = std::process::Command::new("git")
            .args(["log", "--format=%an", "-n", "1", &sha])
            .output()
            .expect("failed to execute process");

        String::from_utf8(commit_message.stdout).unwrap()
    }

    fn get_author_email(sha: &str) -> String {
        let commit_message = std::process::Command::new("git")
            .args(["log", "--format=%ae", "-n", "1", &sha])
            .output()
            .expect("failed to execute process");

        String::from_utf8(commit_message.stdout).unwrap()
    }

    fn get_date(sha: &str) -> String {
        let commit_message = std::process::Command::new("git")
            .args([
                "log",
                "--format=%cd",
                "--date=format:'%Y-%m-%d %H:%M:%S'",
                "-n",
                "1",
                &sha,
            ])
            .output()
            .expect("failed to execute process");

        String::from_utf8(commit_message.stdout).unwrap()
    }
}
