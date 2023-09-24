use crate::config::Config;
use crate::vault::get_staged_files;
use log::info;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Add file or directory content to the current Vault collection
pub fn add(path: String) {
    info!("Staging {path}");

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
        eprintln!("`{path}` is neither a file nor a directory. Aborting.");
        return;
    };

    // remove already staged files from the selection
    let staged = get_staged_files();
    let files_to_add: Vec<String> = files
        .clone()
        .into_iter()
        .filter(|f| !staged.contains(f))
        .collect();
    if files_to_add.is_empty() {
        println!("File(s) already staged. Nothing to do");
        return;
    }

    // add files to the staging zone
    let mut staging_conf_file = OpenOptions::new()
        .append(true)
        .open(Config::staging_file())
        .unwrap();
    if let Err(e) = writeln!(staging_conf_file, "{}", files_to_add.join("\n")) {
        eprintln!("Couldn't write to staging file: {}", e);
    }
}
