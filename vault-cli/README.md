# Vault CLI

CLI tool to interact with the [vault server](../vault-server)

## Installation

```sh
$ # in this folder
$ cargo install --path .
$ vault-cli help
```

## Usage

You can stage files with `vault-cli add`, `vault-cli status` will display the
staged files to be commited (remove them with `vault-cli remove`).

When you are happy with the selection use `vault-cli commit` to upload the files
to the server. The files will be deleted from your system. The
[merkle tree](https://www.wikiwand.com/en/Merkle_tree)'s root hash of the
vaulted files is saved in your local config files (typically
`~/.config/vault/vaults`) to ensure the integrity of your files.

You can then use the `vault-cli list` to list all your vaulted files, and the
`vault-cli download` command can be used to download them again. When a file is
downloaded from the remote server, an integrity check is performed:

- the file and its Merkle Proof are downloaded
- the Merkle tree root's hash is retrieved from the proof
- and is checked against the local hash computed locally during the
  corresponding `commit`

This ensures that the file has not been corrupted by the server, nor the
transportation.

Use `vault-cli delete <VAULT_ID>` to delete a vault. Caution: all files will be
permanently deleted.

```sh
Usage: vault-cli [OPTIONS] <COMMAND>

Commands:
  status    List staged files to be commited to the vault
  add       Add file to the staging area
  remove    Remove file from the staging area
  clear     Remove all file from the staging area
  commit    Commit staged files: upload all staged files to a new vault and delete them
  list      List all files from all vaults
  download  Download file from any vault
  delete    Delete a given vault
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>    Sets a custom config file
  -s, --server <SERVER>  [default: http://vault]
  -n, --no-interaction   
  -h, --help             Print help
```

## Commands Blueprints

### `vault-cli add`

![command: add](../.assets/add.png)

### `vault-cli status`

![command: status](../.assets/status.png)

### `vault-cli commit`

![command: commit](../.assets/commit.png)

### `vault-cli list`

![command: list](../.assets/list.png)

### `vault-cli download`

![command: download](../.assets/download.png)
