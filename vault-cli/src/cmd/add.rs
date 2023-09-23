use crate::config;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub fn add(path: String) {
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
        exit(0);
    };

    let mut file = OpenOptions::new()
        .append(true)
        .open(config::staging_file())
        .unwrap();

    if let Err(e) = writeln!(file, "{}", files.join("\n")) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
