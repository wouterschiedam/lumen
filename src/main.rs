use clap::{command, Parser, Subcommand};
use provider::{openai::OpenAIProvider, phind::PhindProvider};
use reqwest;
use std::error::Error;
use tokio;

mod command;
mod provider;

#[derive(Parser)]
#[command(name = "lumen")]
#[command(about = "A CLI wrapper for AI interactions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Explain {
        #[arg()]
        sha: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Explain { sha } => {
            let provider = PhindProvider::new(client, None);
            let command = command::LumenCommand::new(Box::new(provider));
            let result = command.explain(sha).await?;

            println!("{}", result);
        }
    }

    Ok(())
}
