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
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum ProviderType {
    Openai,
    Phind,
}

#[derive(Subcommand)]
enum Commands {
    Explain {
        #[arg()]
        sha: String,

        #[arg(value_enum, short = 'p', long = "provider", default_value = "phind")]
        provider: ProviderType,

        #[arg(short = 'k', long = "api-key", required_if_eq("provider", "openai"))]
        api_key: Option<String>,
    },
    List {
        #[arg(value_enum, short = 'p', long = "provider", default_value = "phind")]
        provider: ProviderType,

        #[arg(short = 'k', long = "api-key", required_if_eq("provider", "openai"))]
        api_key: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Explain {
            sha,
            provider,
            api_key,
        } => {
            let provider = provider::LumenProvider::new(client, provider, api_key);
            let command = command::LumenCommand::new(provider);
            let result = command.explain(sha).await?;
            println!("{}", result);
        }
        Commands::List { provider, api_key } => {
            let provider = provider::LumenProvider::new(client, provider, api_key);
            let command = command::LumenCommand::new(provider);
            let result = command.list().await?;
            println!("{}", result);
        }
    }

    Ok(())
}
