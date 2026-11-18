#!/bin/sh
set -eu
[ "$#" -eq 2 ] || { echo 'Usage: sh sign.sh APPROVED_SIGNING_KEY PATH_TO_SHA256SUMS' >&2; exit 2; }
[ -f "$2" ] && [ ! -e "$2.sig" ] || { echo 'Check input or existing signature' >&2; exit 1; }
ssh-keygen -Y sign -f "$1" -n mongolia-release "$2"
