use rocket::fs::{NamedFile};
use rocket::serde::json::{json, Value};
use rs_merkle_tree::{utils::crypto::hash, MerkleTree};
use std::{fs, io, path::Path};


use crate::helpers::fs::get_existing_vault_dir;

#[get("/<vault_id>/<file>")]
pub async fn download_file(vault_id: String, file: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("FILES/").join(vault_id).join(file))
        .await
        .ok()
}

#[get("/<vault_id>/<file>/proof")]
pub async fn download_proof(vault_id: String, file: String) -> Value {
    let file_hash = match NamedFile::open(Path::new("FILES/").join(&vault_id).join(file)).await {
        Ok(file) => hash(&fs::read(file.path()).unwrap()),
        Err(_) => {
            return json!({
                "success": false,
                "message": "File not found",
            })
        }
    };

    // TODO: retrieve tree from dump
    let vault_dir = match get_existing_vault_dir(&vault_id) {
        Ok(dir) => dir,
        Err(err) => {
            return json!({
                "success": false,
                "message": err.to_string(),
            })
        }
    };
    let files = fs::read_dir(vault_dir)
        .unwrap()
        .filter(|f| f.as_ref().unwrap().path().is_file())
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let files_hashes: Vec<Vec<u8>> = files
        .into_iter()
        .map(|f| {
            let file = fs::read(f).unwrap();
            hash(&file)
        })
        .collect();

    let tree = MerkleTree::from_leaves(files_hashes);
    let proof = tree.proof(file_hash).unwrap();
    return json!({
        "success": true,
        "proof": proof,
    });
}
