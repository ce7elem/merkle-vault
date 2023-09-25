use crate::helpers::fs::list_files_in_vault;
use rocket::fs::NamedFile;
use rocket::serde::json::{json, Value};
use rs_merkle_tree::{utils::crypto::hash, MerkleTree};
use std::{fs, path::Path};

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

    // TODO: retrieve tree from dump instead of rebuilding it

    let files_hashes: Vec<Vec<u8>> = list_files_in_vault(&vault_id)
        .iter()
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
