#!/bin/sh
# Install an already extracted executable using a separately verified SHA-256.
set -eu
if [ "$#" -lt 2 ] || [ "$#" -gt 3 ]; then
    echo 'Usage: sh install.sh PATH_TO_MN EXPECTED_SHA256 [BIN_DIRECTORY]' >&2
    exit 2
fi
src=$1
digest=$2
dest=${3:-"$HOME/.local/bin"}
case "$digest" in ''|*[!0123456789abcdef]*) echo 'Invalid lowercase SHA-256' >&2; exit 2;; esac
[ "${#digest}" -eq 64 ] || exit 2
mkdir -p -- "$dest"
dest=$(cd -- "$dest" && pwd -P)
tmp=$(mktemp "$dest/.mn-install.XXXXXXXX")
trap 'rm -f -- "$tmp"' EXIT
trap 'exit 1' HUP INT TERM
cat < "$src" > "$tmp"
if command -v sha256sum >/dev/null 2>&1; then
    actual=$(sha256sum < "$tmp")
else
    actual=$(shasum -a 256 < "$tmp")
fi
[ "${actual%% *}" = "$digest" ] || { echo 'Binary checksum mismatch' >&2; exit 1; }
chmod 755 "$tmp"
# link(2) semantics publish a complete file and preserve any existing destination.
# Explicitly reject a directory so ln cannot reinterpret the destination as a parent.
if [ -e "$dest/mn" ] || [ -L "$dest/mn" ]; then echo 'Destination mn already exists' >&2; exit 1; fi
ln "$tmp" "$dest/mn"
echo "Installed $dest/mn"
