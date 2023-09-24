use rs_merkle_tree::{utils::crypto::hash, MerkleTree};
use std::{fs, io, path::Path};
use uuid::Uuid;
use rocket::serde::json::{json, Value};
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};

use crate::helpers::fs::{
    get_existing_vault_dir,
    list_files_in_vault,
};

#[derive(FromForm)]
pub struct Upload<'f> {
    file: TempFile<'f>,
}
#[post("/<vault_id>/upload", data = "<form>")]
pub async fn upload_file(vault_id: String, mut form: Form<Upload<'_>>) -> Value {
    let vault_dir = match get_existing_vault_dir(&vault_id) {
        Ok(dir) => dir,
        Err(err) => {
            return json!({
                "success": false,
                "message": err.to_string(),
            })
        }
    };

    let file = &mut form.file;
    let rname = file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw();
    let name = Path::new(rname.as_str())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let filename = format!("{vault_dir}/{name}");

    match file.persist_to(filename).await {
        Ok(_) => json!({
            "success": true,
            "message": format!("File uploaded to `{vault_id}`"),
        }),
        Err(err) => {
            println!("ERR upload");
            json!({
                "success": false,
                "message": format!("Failed to upload the file: {}", err.to_string()),
            })
        }
    }
}

#[post("/<vault_id>/finalize")]
pub fn finalize_vault(vault_id: String) -> Value {
    let _vault_dir = match get_existing_vault_dir(&vault_id) {
        Ok(dir) => dir,
        Err(err) => {
            return json!({
                "success": false,
                "message": err.to_string(),
            })
        }
    };

    println!("Finalizing {vault_id}");

    let files_hashes: Vec<Vec<u8>> = list_files_in_vault(&vault_id)
        .into_iter()
        .map(|f| {
            let file = fs::read(f).unwrap();
            hash(&file)
        })
        .collect();

    let tree = MerkleTree::from_leaves(files_hashes);

    json!({
        "success": true,
        "message": format!("Finalizing {vault_id}"),
        "tree_root": tree.root_hex().unwrap()
    })
}
