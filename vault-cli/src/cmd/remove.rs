use crate::config::Config;
use log::info;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use crate::vault::get_staged_files;

/// Add file to the current Vault collection
pub fn remove(path: String) {
    info!("Unstaging {path}");

    let files: Vec<String> = if Path::new(&path).is_dir() {
        fs::read_dir(path)
            .unwrap()
            .filter(|f| f.as_ref().unwrap().path().is_file())
            .map(|f| {
                fs::canonicalize(f.unwrap().path())
                    .unwrap()
                    .display()
                    .to_string()
            })
            .collect::<Vec<_>>()
    } else if Path::new(&path).is_file() {
        vec![fs::canonicalize(path).unwrap().display().to_string()]
    } else {
        return; // Noting to do
    };

    // remove selected files from the staging
    let staged = get_staged_files();
    let new_staging: Vec<String> = staged
        .clone()
        .into_iter()
        .filter(|f| !files.contains(f))
        .collect();

    let mut staging_conf_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Config::staging_file())
        .unwrap();
    if let Err(e) = writeln!(staging_conf_file, "{}", new_staging.join("\n")) {
        eprintln!("Couldn't write to staging file: {}", e);
    }
}
