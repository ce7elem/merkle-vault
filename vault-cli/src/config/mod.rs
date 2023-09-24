use dirs;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Config {}

impl Config {
    /// Returns path to the config folder.
    /// Will create the directory if doesn't exists.
    pub fn config_dir() -> PathBuf {
        let path = dirs::config_dir().unwrap().join("vault");
        fs::create_dir_all(&path).unwrap();
        path
    }

    /// Returns path to the `Staged` files.
    /// Will create the file if doesn't exists.
    pub fn staging_file() -> PathBuf {
        let path = Self::config_dir().join("staging");
        if !(Path::new(&path).is_file()) {
            fs::File::create(&path).unwrap();
        }
        return path;
    }

    /// Returns path to the vaults list config files.
    /// Will create the file if doesn't exists.
    pub fn vaults_file() -> PathBuf {
        let path = Self::config_dir().join("vaults");
        if !(Path::new(&path).is_file()) {
            fs::File::create(&path).unwrap();
        }
        return path;
    }
}
