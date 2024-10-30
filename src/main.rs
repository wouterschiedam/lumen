use clap::{command, Parser, Subcommand, ValueEnum};
use reqwest;
use std::error::Error;
use tokio;

mod command;
mod git_commit;
mod provider;

#[derive(Parser)]
#[command(name = "lumen")]
#[command(about = "A CLI wrapper for AI interactions", long_about = None)]
struct Cli {
    #[arg(
        value_enum,
        short = 'p',
        long = "provider",
        env("LUMEN_AI_PROVIDER"),
        default_value = "phind"
    )]
    provider: ProviderType,

    #[arg(
        short = 'k',
        long = "api-key",
        env = "LUMEN_API_KEY",
        required_if_eq("provider", "openai")
    )]
    api_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum ProviderType {
    Openai,
    Phind,
    Groq,
}

#[derive(Subcommand)]
enum Commands {
    Explain {
        #[arg()]
        sha: String,
    },
    List,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let provider = provider::LumenProvider::new(client, cli.provider, cli.api_key);
    let command = command::LumenCommand::new(provider);

    match cli.command {
        Commands::Explain { sha } => command.explain(sha).await?,
        Commands::List => command.list().await?,
    }

    Ok(())
}
