use crate::utils::api::delete_vault;
use crate::vault::delete_vault_local;
use crate::vault::get_all_vaults;
use crate::CliConf;
use std::process::exit;
use dialoguer::Confirm;

/// Delete remote vault
pub fn delete(vault_id: &String, conf: &CliConf) {
    if get_all_vaults()
        .into_iter()
        .find(|v| v == vault_id).is_none() {
        eprintln!("Vault {vault_id} does not exist.");
        exit(-1);
    }

    println!("Vault {vault_id} will be permanently deleted. All files will be lost.");
    if !(conf.no_interaction || Confirm::new().with_prompt("Continue?").interact().unwrap()) {
        println!("Aborting.");
        return;
    }

    delete_vault(vault_id, conf); // remove remote files
    delete_vault_local(vault_id); // update local config
}
