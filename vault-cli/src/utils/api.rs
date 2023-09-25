use crate::CliArgs;
use log::error;
use rs_merkle_tree::MerkleProof;
use serde::Deserialize;
use std::error::Error;
use std::io::Cursor;
use std::process::exit;

#[derive(Deserialize)]
#[allow(dead_code)]
struct ListFilesResponse {
    success: bool,
    message: Option<String>,
    files: Option<Vec<String>>,
}

/// Fetches a list of files stored in a vault.
///
/// # Arguments
///
/// * `vault_id` - The ID of the vault to fetch files from.
/// * `conf` - The CLI configuration containing the HTTP client and API endpoint.
///
/// # Returns
///
/// A vector of strings representing the list of files in the vault.
pub fn fetch_files_in_vault(vault_id: &String, conf: &CliArgs) -> Vec<String> {
    let res = conf
        .http
        .get(format!("{}/{vault_id}/list-files", conf.api_endpoint))
        .send();
    if let Ok(res) = res.unwrap().json::<ListFilesResponse>() {
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

/// Downloads a file from a vault and saves it locally.
///
/// # Arguments
///
/// * `vault_id` - The ID of the vault where the file is stored.
/// * `filename` - The name of the file to download.
/// * `conf` - The CLI configuration containing the HTTP client and API endpoint.
///
/// # Returns
///
/// A `Result` indicating success or an error if there's an issue downloading or saving the file.
pub fn download_file(
    vault_id: &String,
    filename: &String,
    conf: &CliArgs,
) -> Result<(), Box<dyn Error>> {
    let res = conf
        .http
        .get(format!("{}/{vault_id}/{filename}", conf.api_endpoint))
        .send()?;

    let mut file = std::fs::File::create(filename)?;
    let mut content = Cursor::new(res.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ProofResponse {
    success: bool,
    message: Option<String>,
    proof: Option<MerkleProof>,
}

/// Fetches a Merkle proof for a specific file in a vault.
///
/// # Arguments
///
/// * `vault_id` - The ID of the vault where the file is stored.
/// * `filename` - The name of the file for which to fetch the Merkle proof.
/// * `conf` - The CLI configuration containing the HTTP client and API endpoint.
///
/// # Returns
///
/// A `Result` containing the Merkle proof if successful, or an error if there's an issue
/// fetching or parsing the proof.
pub fn fetch_proof_for_file(
    vault_id: &String,
    filename: &String,
    conf: &CliArgs,
) -> Result<MerkleProof, Box<dyn Error>> {
    let res = conf
        .http
        .get(format!("{}/{vault_id}/{filename}/proof", conf.api_endpoint))
        .send()?;

    let res = res
        .json::<ProofResponse>()
        .or(Err("Error while parsing response"))?;
    match res.proof {
        Some(proof) => Ok(proof),
        None => {
            if let Some(message) = res.message {
                return Err(message.into());
            }
            Err("Unable to parse return value".into())
        }
    }
}

pub fn delete_vault(vault_id: &String, conf: &CliArgs) {
    let _ = conf
        .http
        .delete(format!("{}/{vault_id}", conf.api_endpoint))
        .send();
}
