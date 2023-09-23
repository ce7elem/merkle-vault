use crate::config;
use crate::utils::fs::lines_from_file;
use std::process::exit;

pub fn status() {
    let files = lines_from_file(config::staging_file()).unwrap();
    if files.is_empty() {
        println!("Nothing to commit. Add files to staging with the `vault add <path>` command.");
        exit(0);
    }

    println!("Staged files to be commited:");
    for f in &files {
        println!("\t{}", f);
    }
    println!("(use `vault remove <file>` or `vault clear` to unstage)\n");
}
