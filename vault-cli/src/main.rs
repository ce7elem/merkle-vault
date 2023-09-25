use clap::{Parser, Subcommand};
use indicatif::MultiProgress;
use std::path::PathBuf;
use vault_cli::CliArgs;

mod cmd;
mod config;
mod utils;
mod vault;
use cmd::{add, clear, commit, delete, download, list, remove, status};

use indicatif_log_bridge::LogWrapper;

/// Vault CLI
#[derive(Debug, Parser)]
#[command(name = "vault")]
#[command(about = "Keep your files safe in the cloud", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Auto accept confirmation dialogs
    #[arg(short, long, default_value = "http://vault.local:8000")]
    server: String,

    /// Auto accept confirmation dialogs
    #[arg(short = 'y', long, action)]
    no_interaction: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List staged files to be commited to the vault
    Status {},

    /// Add file/directory content to the staging area
    Add { path: String },

    /// Remove file/directory content from the staging area
    Remove { path: String },

    /// Remove all file from the staging area
    Clear {},

    /// Commit staged files: upload all staged files to a new vault and delete them
    Commit {},

    /// List all files from all vaults
    List {},

    /// Download a file from any vault.
    Download {
        file: String,

        /// Specify vault id (needed on duplicates between vaults)
        #[arg(short, long)]
        vault_id: Option<String>,
    },

    /// Delete a given vault
    Delete { vault_id: String },
}

fn main() {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
            .format_timestamp(None)
            .build();

    let args = Cli::parse();
    let conf = crate::CliArgs {
        term_ctx: MultiProgress::new(),
        api_endpoint: args.server,
        http: reqwest::blocking::Client::new(),
        no_interaction: args.no_interaction,
    };

    LogWrapper::new(conf.term_ctx.clone(), logger)
        .try_init()
        .unwrap();

    match args.command {
        Commands::Status {} => status(),
        Commands::Add { path } => add(path),
        Commands::Remove { path } => remove(path),
        Commands::Clear {} => clear(),
        Commands::Commit {} => commit(&conf),
        Commands::List {} => list(&conf),
        Commands::Download { file, vault_id } => download(&file, vault_id, &conf),
        Commands::Delete { vault_id } => delete(&vault_id, &conf),
    }
}
