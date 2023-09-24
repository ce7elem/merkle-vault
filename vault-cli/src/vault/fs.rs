use crate::config::Config;
use crate::utils::fs::lines_from_file;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

pub fn get_all_vaults() -> Vec<String> {
    lines_from_file(Config::vaults_file()).unwrap()
}

pub fn get_root_hash_for_vault(vault_id: &String) -> Result<String, Box<dyn Error>> {
    let file = Config::config_dir().join(format!("{vault_id}.hash"));
    Ok(lines_from_file(file)?
        .first()
        .ok_or("No hash found for vault")?
        .to_string())
}

pub fn save_vault_root_hash(vault_id: &String, hash: &String) -> io::Result<()> {
    let file = Config::config_dir().join(format!("{vault_id}.hash"));
    let mut file = File::create(file)?;
    file.write_all(hash.as_bytes())?;
    Ok(())
}
