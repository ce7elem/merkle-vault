use crate::utils::api::{download_file, fetch_files_in_vault, fetch_proof_for_file};
use crate::vault::{get_all_vaults, get_root_hash_for_vault};
use crate::CliConf;
use log::info;
use std::path::Path;
use std::process::exit;

pub fn download(filename: &String, conf: &CliConf) {
    let files_uri = retrieve_remote_matching_files(filename, conf);

    if files_uri.len() == 0 {
        eprintln!("File {filename} not found in the remote vaults");
        eprintln!("(use `vault list` to list your remote files)");
        exit(-1);
    }
    if files_uri.len() > 1 {
        println!("Multiple files are matching {filename}:");
        for (f, v) in files_uri {
            eprintln!("\t{f} in {v}");
        }
        exit(-1);
    }

    let (vault_id, filename) = &files_uri[0];
    if let Err(err) = download_file(&vault_id, &filename, conf) {
        eprintln!("Something went wrong while downloading the file: {err}");
        exit(-1);
    }

    let proof = match fetch_proof_for_file(&vault_id, &filename, conf) {
        Ok(proof) => proof,
        Err(err) => {
            eprintln!("Something went wrong while fetching proof: {err}");
            exit(-1);
        }
    };
    let local_root_hash = get_root_hash_for_vault(&vault_id).unwrap();
    if proof.compute_root_hex().unwrap() != local_root_hash {
        eprintln!("ERROR: File alteration detected.");
        exit(-1);
    }

    info!("'{filename}' downloaded successfully.");
}

fn retrieve_remote_matching_files(filename: &String, conf: &CliConf) -> Vec<(String, String)> {
    let mut matches = Vec::<(String, String)>::new();

    for vault_id in get_all_vaults() {
        info!("Searching in vault {vault_id}");
        if let Some(uri) = fetch_files_in_vault(&vault_id, conf).iter().find(|f| {
            info!("\t- {f}");
            Path::new(f).file_name().unwrap().to_str() == Some(filename)
        }) {
            matches.push((vault_id.clone(), uri.clone()));
        }
    }
    return matches;
}
