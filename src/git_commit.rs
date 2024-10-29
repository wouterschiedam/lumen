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

        let mut commit_message = String::from_utf8(commit_message.stdout).unwrap();
        commit_message.pop(); // remove trailing newline from echo
        commit_message.pop();

        commit_message
    }

    fn get_author_name(sha: &str) -> String {
        let name = std::process::Command::new("git")
            .args(["log", "--format=%an", "-n", "1", &sha])
            .output()
            .expect("failed to execute process");

        let mut name = String::from_utf8(name.stdout).unwrap();
        name.pop(); // remove trailing newline from echo

        name
    }

    fn get_author_email(sha: &str) -> String {
        let email = std::process::Command::new("git")
            .args(["log", "--format=%ae", "-n", "1", &sha])
            .output()
            .expect("failed to execute process");

        let mut email = String::from_utf8(email.stdout).unwrap();
        email.pop(); // remove trailing newline from echo

        email
    }

    fn get_date(sha: &str) -> String {
        let date = std::process::Command::new("git")
            .args([
                "log",
                "--format=%cd",
                "--date=format:%Y-%m-%d %H:%M:%S",
                "-n",
                "1",
                &sha,
            ])
            .output()
            .expect("failed to execute process");

        let mut date = String::from_utf8(date.stdout).unwrap();
        date.pop(); // remove trailing newline from echo

        date
    }
}
