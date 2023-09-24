use crate::config::Config;
use std::fs;

/// Clean staging (ie. remove all the files from the staging area)
pub fn clear() {
    if let Err(e) = fs::remove_file(Config::staging_file()) {
        eprintln!("Couldn't delete the staging file: {}", e);
    }
}
