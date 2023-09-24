#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

mod helpers;
mod routes;
use routes::{
    create_vault,
    upload_file,
    finalize_vault,
    list_vault_files,
    delete_vault,
    download_file,
    download_proof,
};

#[get("/")]
fn index() -> Value {
    json!({
        "success": true,
        "message":"Vault is online.",
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
            download_proof,
        ],
    )
}
