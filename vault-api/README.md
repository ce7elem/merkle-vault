# Vault Server

## Installation

```sh
$ # in this folder

# run in debug mode
$ cargo run

# for production
$ cargo build --release
$ ./target/release/vault-api
```

## Endpoints

- `POST /new-vault`: Create a new vault uuid, its associate folder and return
  the id.
- `POST /<vault_id>/upload`: Upload a file to the specified vault. TODO: verify
  the vault is not already closed.
- `POST /<vault_id>/finalize`: Compute the merkle tree of the filesystem. TODO:
  lock vault.
- `GET /<vault_id>/list-files`: Returns a list of filenames in the vault.
- `DELETE /<vault_id>`: Removes the associated folder.
- `GET /<vault_id>/<filename>`: Download file from the vault.
- `GET /<vault_id>/<filename>/proof`: Returns the merkle proof for the file.
