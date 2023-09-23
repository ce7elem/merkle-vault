use crate::config;
use crate::utils::fs::lines_from_file;
use crate::CliConf;
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::{error, info};
use rs_merkle_tree::utils::crypto::{hash, Hash};
use rs_merkle_tree::MerkleTree;
use serde::Deserialize;
use std::fmt::Write;
use std::fs;
use std::process::exit;
use std::fs::OpenOptions;
use std::io::Write as ioWrite;

#[derive(Deserialize)]
struct Response {
    success: bool,
    message: String,
    tree_root: Option<String>,
}

pub fn commit(conf: &CliConf) {
    let files = lines_from_file(config::staging_file()).unwrap();
    if files.is_empty() {
        println!("Nothing to commit. Add files to staging with the `add` command.");
        exit(0);
    }

    println!("The following files will be uploaded :");
    for f in &files {
        println!("\t{}", f);
    }
    if !(Confirm::new().with_prompt("Continue?").interact().unwrap()) {
        println!("Aborting.");
        return;
    }

    let collection = create_collection(conf);
    upload_files(&files, &collection, conf);
    let remote_root = finalize_upload(&collection, conf);

    let local_root = compute_local_root(&files, conf);

    info!("Local root hash:  {local_root}");
    info!("Remote root hash: {remote_root}");
    if remote_root != local_root {
        error!("Remote FS seems corrupted.");
        abort_gracefully(&collection, conf);
    }

    let _ = fs::write(".vault", local_root);
}

fn compute_local_root(files: &Vec<String>, conf: &CliConf) -> String {
    let pb = conf
        .term_ctx
        .add(ProgressBar::new(files.len().clone().try_into().unwrap()));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {msg} {pos:>5}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.0}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    let files_hashes: Vec<Hash> = files
        .into_iter()
        .enumerate()
        .map(|(i, f)| {
            let file = fs::read(f).unwrap();
            pb.set_position(i.try_into().unwrap());
            pb.set_message(f.clone());
            hash(&file)
        })
        .collect();

    pb.finish_with_message("all hashed computed");

    let tree = MerkleTree::from_leaves(files_hashes);
    tree.root_hex().unwrap()
}

fn upload_files(files: &Vec<String>, collection: &String, conf: &CliConf) {
    let client = reqwest::blocking::Client::new();
    let pb = conf
        .term_ctx
        .add(ProgressBar::new(files.len().clone().try_into().unwrap()));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {msg} {pos:>5}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.0}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    for (i, f) in files.clone().iter().enumerate() {
        let form = reqwest::blocking::multipart::Form::new()
            .file("file", f.clone())
            .unwrap();
        let upload = client
            .post(format!("{}/{collection}/upload", conf.api_endpoint))
            .multipart(form)
            .send();

        if let Ok(res) = upload.unwrap().json::<Response>() {
            if res.success == false {
                pb.abandon_with_message("Upload failed");
                error!("Something went wrong during the upload: {}.", res.message);
                abort_gracefully(collection, conf);
            }
        }
        pb.set_position(i.try_into().unwrap());
        pb.set_message(f.clone());
    }
    pb.finish_with_message("all files uploaded");
}

fn create_collection(conf: &CliConf) -> String {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct NewVaultResponse {
        success: bool,
        message: String,
        vault_id: Option<String>,
    }

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(format!("{}/new-vault", conf.api_endpoint))
        .send();
    if let Ok(res) = res.unwrap().json::<NewVaultResponse>() {
        match res.vault_id {
            Some(vault_id) => {
                let mut vaults_file = OpenOptions::new()
                    .append(true)
                    .open(config::vaults_file())
                    .unwrap();

                if let Err(e) = writeln!(vaults_file, "{}", vault_id) {
                    eprintln!("Error while saving vault_id: {e}");
                }
                return vault_id;
            },
            None => {
                error!("Something went wrong: {}", res.message);
                exit(-1);
            }
        }
    }
    error!("Something went wrong.");
    exit(-1);
}

fn finalize_upload(collection: &String, conf: &CliConf) -> String {
    let client = reqwest::blocking::Client::new();
    let remote_files = client
        .post(format!("{}/{collection}/finalize", conf.api_endpoint))
        .send();
    match remote_files.unwrap().json::<Response>() {
        Ok(res) => {
            if let Some(root_hash) = res.tree_root {
                return root_hash;
            } else {
                error!("Upload failed: {}", res.message);
                abort_gracefully(collection, conf);
                unreachable!()
            }
        }
        Err(err) => {
            error!("Upload error: {err:?}");
            abort_gracefully(collection, conf);
            unreachable!()
        }
    }
}

fn abort_gracefully(collection: &String, conf: &CliConf) {
    let client = reqwest::blocking::Client::new();
    error!("Exitting gracefully...");
    error!("Resetting remote FS.");
    let _ = client
        .delete(format!("{}/{collection}", conf.api_endpoint))
        .send();
    exit(-1);
}
