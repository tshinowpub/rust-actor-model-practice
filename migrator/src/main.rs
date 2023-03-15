use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::exit;
use dynamodb_client::client::Client;

use crate::command::list::List as ListCommand;
use crate::command::migrate::Migrate as MigrateCommand;
use crate::command::migrate_type::MigrateType;
use crate::command::reset::Reset as ResetCommand;
use crate::settings::Settings;

mod command;
mod parser;
mod settings;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Migrate DynamoDB script.
    Migrate {
        command: MigrateType,

        #[arg(short, long, required = false)]
        path: Option<PathBuf>,
    },
    /// Display command list.
    List {},
    /// Create migrate file.
    Create {},
    /// Reset migration.
    Reset {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    let settings = Settings::new().map_err(|error| anyhow!(error))?;

    let client = settings
        .dynamodb()
        .uri()
        .map(|uri| Client::new(uri))
        .unwrap();

    match &cli.command {
        Some(Commands::Migrate { command, path }) => {
            let migrate = MigrateCommand::new(client);

            let result = migrate.execute(command, path.as_ref()).await;

            match result {
                Ok(output) => {
                    println!("{}", output.message());

                    exit(*(output.exit_code()) as i32);
                }
                Err(error) => {
                    println!("{}", error);

                    exit(1);
                }
            }
        }
        Some(Commands::List {}) => {
            let list = ListCommand::new();

            let output = list.execute().await;

            println!("{}", output.message());

            exit(*(output.exit_code()) as i32);
        }
        Some(Commands::Create {}) => exit(0),
        Some(Commands::Reset {}) => {
            let reset = ResetCommand::new(client);

            let result = reset.execute().await;
            match result {
                Ok(output) => {
                    println!("{}", output.message());

                    exit(*(output.exit_code()) as i32);
                }
                Err(error) => {
                    println!("{}", error);

                    exit(1);
                }
            }
        }
        None => {
            if let Some(name) = cli.name.as_deref() {
                println!("Command {} was not found.", name);
            }

            exit(0)
        }
    }
}
