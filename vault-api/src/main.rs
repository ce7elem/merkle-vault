use std::fs;
use uuid::Uuid;
#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::{json, Value};

use rs_merkle_tree::utils::crypto::hash;
use rs_merkle_tree::MerkleTree;
use std::io;
use std::path::Path;

#[get("/")]
fn index() -> Value {
    json!({
        "success": true,
        "message":"Vault is online.",
    })
}

#[post("/new-vault")]
fn create_vault() -> Value {
    let fsid = Uuid::new_v4();
    match fs::create_dir(format!("./FILES/{fsid}")) {
        Ok(_) => json!({
            "success": true,
            "message": "FS created.",
            "vault_id": fsid.to_string()
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
#[post("/<fsid>/upload", data = "<form>")]
async fn upload_file(fsid: String, mut form: Form<Upload<'_>>) -> Value {
    let vault_dir = format!("./FILES/{fsid}");
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
            "message": format!("File uploaded to `{fsid}`"),
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

#[post("/<fsid>/finalize")]
fn finalize_vault(fsid: String) -> Value {
    let vault_dir = format!("./FILES/{fsid}");
    if !Path::new(&vault_dir).is_dir() {
        return json!({
            "success": false,
            "message": "FS does not exists",
        });
    }
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

    json!({
        "success": true,
        "message": format!("Finalizing {fsid}"),
        "tree_root": tree.root_hex().unwrap()
    })
}

#[get("/<fsid>/list-files")]
fn list_vault_files(fsid: String) -> Value {
    let vault_dir = format!("./FILES/{fsid}");
    if !Path::new(&vault_dir).is_dir() {
        return json!({
            "success": false,
            "message": "FS does not exists",
        });
    }
    let files = fs::read_dir(vault_dir)
        .unwrap()
        .filter(|f| f.as_ref().unwrap().path().is_file())
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    json!({
        "success": true,
        "files": files,
    })
}

#[delete("/<fsid>")]
fn delete_vault(fsid: String) -> Value {
    let vault_dir = format!("./FILES/{fsid}");
    if !Path::new(&vault_dir).is_dir() {
        return json!({
            "success": false,
            "message": "FS does not exists",
        });
    }

    fs::remove_dir_all(vault_dir).unwrap();

    json!({
        "success": true,
        "message": format!("Deleted {fsid}"),
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
            delete_vault
        ],
    )
}
