use crate::utils::api::fetch_files_in_vault;
use crate::vault::get_all_vaults;
use crate::CliConf;

pub fn list(conf: &CliConf) {
    for vault_id in get_all_vaults() {
        println!("Files in vault {vault_id}:");
        for f in fetch_files_in_vault(&vault_id, conf) {
            println!("\t{f}");
        }
        println!("");
    }
}
