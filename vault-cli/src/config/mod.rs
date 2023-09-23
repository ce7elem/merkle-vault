use dirs;
use std::fs;
use std::path::{Path, PathBuf};

/// Returns path to the config folder.
/// Will create the directory if doesn't exists.
pub fn config_base() -> PathBuf {
    let path = dirs::config_dir().unwrap().join("vault");
    fs::create_dir_all(&path).unwrap();
    path
}

/// Returns path to the `Staged` files.
/// Will create the file if doesn't exists.
pub fn staging_file() -> PathBuf {
    let path = config_base().join("staging");
    if !(Path::new(&path).is_file()) {
        fs::File::create(&path).unwrap();
    }
    return path;
}
