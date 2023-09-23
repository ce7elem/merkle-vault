use clap::{Parser, Subcommand};
use indicatif::MultiProgress;
use log::info;

mod cmd;
mod config;
mod utils;
use cmd::{add, commit, status};

use indicatif_log_bridge::LogWrapper;

/// Vault CLI
#[derive(Debug, Parser)]
#[command(name = "vault")]
#[command(about = "Keep your files safe in the cloud", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List staged files to be commited to the Vault
    Status {},

    /// Add file to the current Vault collection
    Add { path: String },

    /// Remove file from the current Vault
    // Remove { path: String },

    /// Commit the vault: Upload all staged files to the server and Delete them
    Commit {},

    /// Download file from any Vault
    Download { file: Option<String> },

    /// List all files from all Vaults
    List {},
}

pub struct CliConf {
    term_ctx: MultiProgress,
    api_endpoint: String,
}
fn main() {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
            .format_timestamp(None)
            .build();
    let conf = CliConf {
        term_ctx: MultiProgress::new(),
        api_endpoint: "http:localhost:8000/".into(),
    };
    LogWrapper::new(conf.term_ctx.clone(), logger)
        .try_init()
        .unwrap();

    let args = Cli::parse();

    match args.command {
        Commands::Status {} => status(),
        Commands::Add { path } => add(path),
        Commands::Commit {} => commit(&conf),

        Commands::Download { file } => {
            info!("Downloading {file:?}");
        }

        Commands::List {} => {
            info!("Listing files");
        }
    }
}
