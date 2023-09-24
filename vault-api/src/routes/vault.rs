use crate::helpers::fs::{get_existing_vault_dir, list_files_in_vault};
use rocket::serde::json::{json, Value};
use std::{fs, path::Path};
use uuid::Uuid;

#[post("/new-vault")]
pub fn create_vault() -> Value {
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

#[delete("/<vault_id>")]
pub fn delete_vault(vault_id: String) -> Value {
    let vault_dir = match get_existing_vault_dir(&vault_id) {
        Ok(dir) => dir,
        Err(err) => {
            return json!({
                "success": false,
                "message": err.to_string(),
            })
        }
    };

    fs::remove_dir_all(vault_dir).unwrap();

    json!({
        "success": true,
        "message": format!("Deleted {vault_id}"),
    })
}

#[get("/<vault_id>/list-files")]
pub fn list_vault_files(vault_id: String) -> Value {
    json!({
        "success": true,
        "files": list_files_in_vault(&vault_id)
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
