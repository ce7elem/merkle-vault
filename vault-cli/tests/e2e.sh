#!/bin/bash
set -e

echo "[*] Setting up test context"
rm -rf /tmp/vault-end-to-end-tests
mkdir -p /tmp/vault-end-to-end-tests
cd /tmp/vault-end-to-end-tests

for f in {a..z} {A..Z} {0..99} # creates 152 files
do
    echo "this is file $f" > "$f.txt"
done


echo "[*] Testing 'add' command"

vault-cli -s "$VAULT_ENDPOINT" add /tmp/vault-end-to-end-tests
# `status` cmd adds 2 lines of display
[ "$(vault-cli -s "$VAULT_ENDPOINT" status | wc -l)" = "156" ] \
    && echo "  [+] Staged 152 files" \
    || exit 1;


echo "[*] Testing 'commit' command"

[ "$(vault-cli -s "$VAULT_ENDPOINT" list | wc -l)" = "0" ] && echo "[+] Uploaded 152 files" || exit 1;

vault-cli -s "$VAULT_ENDPOINT" --no-interaction commit

[ "$(ls | wc -l)" = "0" ] \
  && echo "[+] Committed files removed successfully from local fs" \
  || exit 1


echo "[*] Testing 'list' command"
# `list` cmd adds 2 lines of display
[ "$(vault-cli -s "$VAULT_ENDPOINT" list | wc -l)" = "154" ] \
  && echo "[+] Uploaded 152 files" \
  || exit 1


echo "[*] Testing 'download' command"

vault-cli -s "$VAULT_ENDPOINT" download 42.txt
[ "$(cat 42.txt)" = "this is file 42" ] \
  && echo "[+] File download successful" \
  || exit 1

vault-cli -s "$VAULT_ENDPOINT" --no-interaction delete $(cat $HOME/.config/vault/vaults)
[ "$(vault-cli -s "$VAULT_ENDPOINT" list | wc -l)" = "0" ] \
  && echo "[+] File download successful" \
  || exit 1
