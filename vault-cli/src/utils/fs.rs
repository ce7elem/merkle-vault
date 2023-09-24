use crate::config;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn get_all_vaults() -> Vec<String> {
    lines_from_file(config::vaults_file()).unwrap()
}
