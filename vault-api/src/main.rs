use std::fs;
use uuid::Uuid;
#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::{json, Value};

use rocket::fs::NamedFile;
use rs_merkle_tree::utils::crypto::hash;
use rs_merkle_tree::MerkleTree;
use std::io;
use std::path::Path;
mod helpers;

#[get("/")]
fn index() -> Value {
    json!({
        "success": true,
        "message":"Vault is online.",
    })
}

#[post("/new-vault")]
fn create_vault() -> Value {
    let vault_id = Uuid::new_v4();
    match fs::create_dir(format!("./FILES/{vault_id}")) {
        Ok(_) => json!({
            "success": true,
            "message": "FS created.",
            "vault_id": vault_id.to_string()
        }),
        Err(err) => json!({
            "success": false,
            "message": format!("Something went wrong while creating the fs entry: {}", err.to_string()),
        }),
    }
}

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>,
}
#[post("/<vault_id>/upload", data = "<form>")]
async fn upload_file(vault_id: String, mut form: Form<Upload<'_>>) -> Value {
    let vault_dir = format!("./FILES/{vault_id}");
    if !Path::new(&vault_dir).is_dir() {
        return json!({
            "success": false,
            "message": "FS does not exists",
        });
    }

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
fn finalize_vault(vault_id: String) -> Value {
    let vault_dir = format!("./FILES/{vault_id}");
    if !Path::new(&vault_dir).is_dir() {
        return json!({
            "success": false,
            "message": "FS does not exists",
        });
    }

    println!("Finalizing {vault_id}");

    let files_hashes: Vec<Vec<u8>> = helpers::fs::list_files_in_vault(&vault_id)
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

#[get("/<vault_id>/list-files")]
fn list_vault_files(vault_id: String) -> Value {
    let vault_dir = format!("./FILES/{vault_id}");
    if !Path::new(&vault_dir).is_dir() {
        return json!({
            "success": false,
            "message": "FS does not exists",
        });
    }

    json!({
        "success": true,
        "files": helpers::fs::list_files_in_vault(&vault_id)
            .iter().map(|f| {
                    Path::new(f)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                }).collect::<Vec<String>>()
    })
}

#[get("/<vault_id>/<file>")]
async fn download_file(vault_id: String, file: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("FILES/").join(vault_id).join(file))
        .await
        .ok()
}

#[get("/<vault_id>/<file>/proof")]
async fn download_file_proof(vault_id: String, file: String) -> Value {
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
    let vault_dir = format!("./FILES/{vault_id}");
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

#[delete("/<vault_id>")]
fn delete_vault(vault_id: String) -> Value {
    let vault_dir = format!("./FILES/{vault_id}");
    if !Path::new(&vault_dir).is_dir() {
        return json!({
            "success": false,
            "message": "FS does not exists",
        });
    }

    fs::remove_dir_all(vault_dir).unwrap();

    json!({
        "success": true,
        "message": format!("Deleted {vault_id}"),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            create_vault,
            upload_file,
            finalize_vault,
            list_vault_files,
            delete_vault,
            download_file,
            download_file_proof,
        ],
    )
}
