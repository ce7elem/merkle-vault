use clap::{Parser, Subcommand};
use indicatif::MultiProgress;
use vault_cli::CliConf;

mod cmd;
mod config;
mod utils;
mod vault;
use cmd::{add, clear, commit, download, list, remove, status};

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
    Remove { path: String },

    /// Remove all file from the staging zone
    Clear {},

    /// Commit the vault: Upload all staged files to the server and Delete them
    Commit {},

    /// List all files from all Vaults
    List {},

    /// Download file from any Vault
    Download { file: String },
}

fn main() {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
            .format_timestamp(None)
            .build();
    let conf = crate::CliConf {
        term_ctx: MultiProgress::new(),
        api_endpoint: "http:localhost:8000".into(),
        http: reqwest::blocking::Client::new(),
    };
    LogWrapper::new(conf.term_ctx.clone(), logger)
        .try_init()
        .unwrap();

    let args = Cli::parse();

    match args.command {
        Commands::Status {} => status(),
        Commands::Add { path } => add(path),
        Commands::Remove { path } => remove(path),
        Commands::Clear {} => clear(),
        Commands::Commit {} => commit(&conf),
        Commands::List {} => list(&conf),
        Commands::Download { file } => download(&file, &conf),
    }
}
