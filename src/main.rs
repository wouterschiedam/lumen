use clap::{command, Parser, Subcommand};
use keyring::Entry;
use provider::openai::OpenAIProvider;
use reqwest;
use std::error::Error;
use tokio;

mod command;
mod provider;

const SERVICE_NAME: &str = "lumen";

#[derive(Parser)]
#[command(name = "lumen")]
#[command(about = "A CLI wrapper for AI interactions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, env = "API_KEY", hide_env_values = true)]
    api_key: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure API key
    Configure {
        /// Set the API key
        #[arg(short, long)]
        api_key: String,
    },
    /// Generate a text completion
    Explain {
        #[arg()]
        sha: String,
    },
}

fn get_api_key() -> Result<String, Box<dyn Error>> {
    let entry = Entry::new(SERVICE_NAME, "default")?;
    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(_) => Err(
            "API key not found. Please configure it using 'lumen configure --api-key YOUR_KEY'"
                .into(),
        ),
    }
}

fn save_api_key(key: &str) -> Result<(), Box<dyn Error>> {
    let entry = Entry::new(SERVICE_NAME, "default")?;
    entry.set_password(key)?;
    println!("API key saved successfully!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    match cli.command {
        Commands::Configure { api_key } => {
            save_api_key(&api_key)?;
        }
        Commands::Explain { sha } => {
            let api_key = cli.api_key.unwrap_or_else(|| get_api_key().unwrap());
            let provider = OpenAIProvider::new(client, api_key);
            let command = command::LumenCommand::new(Box::new(provider));
            let result = command.explain(sha).await?;

            println!("{}", result);
        }
    }

    Ok(())
}
