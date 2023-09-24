use crate::CliConf;
use log::error;
use serde::Deserialize;
use std::error::Error;
use std::process::exit;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Response {
    success: bool,
    message: Option<String>,
    files: Option<Vec<String>>,
}

pub fn fetch_files_in_vault(vault_id: &String, conf: &CliConf) -> Vec<String> {
    let res = conf
        .http
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

use std::io::Cursor;
pub fn download_file(
    vault_id: &String,
    filename: &String,
    conf: &CliConf,
) -> Result<(), Box<dyn Error>> {
    let res = conf
        .http
        .get(format!("{}/{vault_id}/{filename}", conf.api_endpoint))
        .send()
        .unwrap();

    let mut file = std::fs::File::create(filename)?;
    let mut content = Cursor::new(res.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
