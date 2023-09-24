use std::fs;
use std::io;
use std::path::Path;

pub fn list_files_in_vault(vault_id: &String) -> Vec<String> {
    let vault_dir = format!("./FILES/{vault_id}");
    fs::read_dir(vault_dir)
        .unwrap()
        .filter(|f| f.as_ref().unwrap().path().is_file())
        .map(|res| {
            res.map(|e| {
                Path::new(e.path().to_str().unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap()
}
