# Vault

## Abstact

This software allows the user the store files on a remote server and download
them later. Checks are systematically performed on the files to ensure their
integrity.

The software consists in two parts:

- the server which hosts the files in a structured way, and exposes REST
  endpoints to interacts with.
- a CLI tool, that interacts with the server in a user-friendly manner.

### Notions

One can upload a set of files (via `add` and `commit` commands) to the server.
This action will create a **vault** containing the given files.

A **vault** is a _read only_ collection of files. Once commited, it will not be
possible to add or remove files from it. Possible actions on a vault are:

- `list` its content
- `download` file from it
- `delete` the vault. Caution, all files will be lost, be sure to have
  downloaded important files before.

Because of this _read only_ property of a vault, and to make it more user
frienly, the CLI tool mimics the `git` behaviour :

- files can be _added_ and _removed_ from a _staging_ area
- once the user is happy with the selection, the staged files can be _commited_
  to a new vault: they will be uploaded and remove from the client filesystem.
  Note: before being removed, the root hash of the merkle tree generated from
  the files will be saved for later integrity checks.
- then, it is possible to _download_ files from a vault. Under the hood, both
  the file and its Merkle proof are downloaded, and the root hash computed from
  the proof is checked against the previously saved one.
- finally, the user can delete the vault and its files from the remote fs.

## Motivation

> Imagine a client has a large set of potentially small files {F0, F1, …, Fn}
> and wants to upload them to a server and then delete its local copies. The
> client wants, however, to later download an arbitrary file from the server and
> be convinced that the file is correct and is not corrupted in any way (in
> transport, tampered with by the server, etc.).

## Getting Started

### REST API Server

```sh
$ cd vault-api/
$ cargo run # will run the server in debug mode
```

### CLI client

```sh
$ cd vault-cli/
$ cargo install --path .
$ vault-cli status # all done
```

### Dockerized Testing Environment

Let's use docker to test the product without risking detroying your computer:

```sh
$ # The following will build images for both the client and the server, and create
$ # a virtual network between them.
$ # It will also run end to end tests. Do not mind the messy stdout coming from
$ # the testing client. The server will stay up.
$ docker compose -f ./docker-compose.yaml up
$
$ # Now run the client
$ docker run -it --net vault_vaultnet vault-client:latest /bin/bash
root@docker $ vault-cli help
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

The client is configured by default to use the dockerized REST API Server. No
need to specify it with `--server`.

## Design

### CLI

The workflow is similar to `git`.

You can stage files with `vault-cli add`, `vault-cli status` will display the
staged files to be commited (remove them with `vault-cli remove`).

When you are happy with the selection use `vault-cli commit` to upload the files
to the server. The files will be deleted from your system. The
[merkle tree](https://www.wikiwand.com/en/Merkle_tree)'s root hash of the
vaulted files is saved in your local config files (typically
`~/.config/vault/vaults`) to ensure the integrity of your files.

You can then use the `vault-cli list` to list all your vaulted files, and the
`vault-cli download [--vault-id <VAULT>] <FILE>` command can be used to download
them again. When a file is downloaded from the remote server, an integrity check
is performed:

- the file and its Merkle Proof are downloaded
- the Merkle tree root's hash is retrieved from the proof
- and is checked against the local hash computed locally during the
  corresponding `commit`

This ensures that the file has not been corrupted by the server, nor the
transportation.

Use `vault-cli delete <VAULT_ID>` to delete a vault. Caution: all files will be
permanently deleted.

#### CLI Commands Blueprints

##### `vault-cli add <FILE>|<DIRECTORY>`

Stage file or files in a directory.

![command: add](./.assets/add.png)

##### `vault-cli remove <FILE>|<DIRECTORY>`

Unstage file or files in a directory.

##### `vault-cli status`

Show staged files.

![command: status](./.assets/status.png)

##### `vault-cli commit`

Creates a vault, upload staged files into it, genreate merkle tree and store the
root hash in `~/.config/vault/<vault_id>.hash`, and remove the local files. The
vault id is append in `~/.config/vault/vaults`.

![command: commit](./.assets/commit.png)

##### `vault-cli list`

Lists all the files amongs user's vaults (User vaults ids are stored in
`~/.config/vault/vaults`).

![command: list](./.assets/list.png)

##### `vault-cli download [--vault-id <VAULT>] <FILE>`

Downloads a file from any vault.

If `--vault-id` is not specifed `download` will search for the filename in all
the vaults. On duplicates filename amongst multiple vaults, it will be needed to
specify it.

![command: download](./.assets/download.png)

### REST API Server

The server provide a simplistic API to interact with the linux filesystem. Each
vault is represented as a folder, and every files in it.

The exposed routes are:

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

## Next Steps

- [ ] Authentication
- [ ] Bulk file upload/download
- [ ] Named Vault / user-frienly vault name
- [ ] Proper config file (persist server endpoint, ...)
- [ ] Option to encrypt files before vaulting them
- [ ] "Writable" Vault (keep all files hash? download the whole vault > add file
      > commit it?)
- [ ] CLI Autocompletion
- [ ] add/remove/download multiple files (directory supported)
