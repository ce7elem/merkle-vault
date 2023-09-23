use crate::config;
use crate::utils::fs::lines_from_file;
use crate::CliConf;
use log::error;
use serde::Deserialize;
use std::process::exit;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Response {
    success: bool,
    message: Option<String>,
    files: Option<Vec<String>>,
}

fn fetch_files_in_vault(vault_id: &String, conf: &CliConf) -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("{}/{vault_id}/list-files", conf.api_endpoint))
        .send();
    if let Ok(res) = res.unwrap().json::<Response>() {
        match res.files {
            Some(files) => return files,
            None => {
                error!("Something went wrong while listing remote files.");
                if let Some(msg) = res.message {
                    error!("Server response: {}", msg);
                }
                exit(-1);
            }
        }
    }
    error!("Something went wrong.");
    exit(-1);
}

fn get_all_vaults() -> Vec<String> {
    lines_from_file(config::vaults_file()).unwrap()
}

pub fn list(conf: &CliConf) {
    for vault_id in get_all_vaults() {
        println!("Files in vault {vault_id}:");
        for f in fetch_files_in_vault(&vault_id, conf) {
            println!("\t{f}");
        }
        println!("");
    }
}
