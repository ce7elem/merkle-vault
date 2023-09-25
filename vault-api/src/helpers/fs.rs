use std::{error::Error, fs, io, path::Path};

pub fn list_files_in_vault(vault_id: &String) -> Vec<String> {
    let vault_dir = get_existing_vault_dir(vault_id).unwrap();

    fs::read_dir(vault_dir)
        .unwrap()
        .filter(|f| f.as_ref().unwrap().path().is_file())
        .map(|res| {
            res.map(|e| {
                Path::new(e.path().to_str().unwrap())
                    .to_str()
                    .unwrap()
                    .to_string()
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap()
}

pub fn get_existing_vault_dir(vault_id: &String) -> Result<String, Box<dyn Error>> {
    if vault_id != Path::new(vault_id).file_name().unwrap().to_str().unwrap() {
        return Err("Provided vault_id is invalid".into());
    };
    let vault_dir = format!("./FILES/{vault_id}");
    if Path::new(&vault_dir).is_dir() {
        Ok(vault_dir.to_owned())
    } else {
        Err("Vault does not exists.".into())
    }
}
