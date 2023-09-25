use crate::config::Config;
use crate::utils::fs::lines_from_file;
use std::error::Error;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{self, Write};

/// Returns a list of all vault names read from the vaults configuration file.
///
/// # Returns
///
/// A `Vec<String>` containing the names of all the vaults.
pub fn get_all_vaults() -> Vec<String> {
    lines_from_file(Config::vaults_file()).unwrap()
}

/// Retrieves the root hash associated with a specific vault by its ID.
///
/// # Arguments
///
/// * `vault_id` - The ID of the vault for which to retrieve the root hash.
///
/// # Returns
///
/// A `Result<String, Box<dyn Error>>` containing the root hash if found,
/// or an error if the hash is not found or if there's an error reading the file.
pub fn get_root_hash_for_vault(vault_id: &String) -> Result<String, Box<dyn Error>> {
    let file = Config::config_dir().join(format!("{vault_id}.hash"));
    Ok(lines_from_file(file)?
        .first()
        .ok_or("No hash found for vault")?
        .to_string())
}

/// Saves the root hash associated with a specific vault to a file.
///
/// # Arguments
///
/// * `vault_id` - The ID of the vault for which to save the root hash.
/// * `hash` - The root hash to be saved.
///
/// # Returns
///
/// An `Result<()>` indicating success or an error if there's an issue
/// creating or writing to the file.
pub fn save_vault_root_hash(vault_id: &String, hash: &String) -> io::Result<()> {
    let file = Config::config_dir().join(format!("{vault_id}.hash"));
    let mut file = File::create(file)?;
    file.write_all(hash.as_bytes())?;
    Ok(())
}

/// Returns a list of the staged files.
///
/// # Returns
///
/// A `Vec<String>` containing the names of staged files.
pub fn get_staged_files() -> Vec<String> {
    lines_from_file(Config::staging_file()).unwrap()
}

pub fn clear_staging() {
    if let Err(e) = fs::remove_file(Config::staging_file()) {
        eprintln!("Couldn't delete the config staging file: {}", e);
    }
}

pub fn delete_vault_local(vault_id: &String) {
    let vault_root_hash = Config::config_dir().join(format!("{vault_id}.hash"));
    if let Err(e) = fs::remove_file(vault_root_hash) {
        eprintln!("Couldn't delete vault root hash: {}", e);
    }

    // update vaults list
    let vaults = lines_from_file(Config::vaults_file()).unwrap();
    let new_vaults: Vec<String> = vaults
        .clone()
        .into_iter()
        .filter(|v| !vaults.contains(v))
        .collect();

    let mut vaults_conf_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Config::staging_file())
        .unwrap();
    if let Err(e) = writeln!(vaults_conf_file, "{}", new_vaults.join("\n")) {
        eprintln!("Couldn't write to staging file: {}", e);
    }
}
