#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DESMOS_HOME="$SCRIPT_DIR/.desmos"
KEYRING_PASS=pass1234


desmos() {
	"$SCRIPT_DIR/desmos" --home="$DESMOS_HOME" "$@"

	# Wait tx including block
	sleep 2
}

echo $KEYRING_PASS | desmos tx "$@" --keyring-backend=file -b=sync -y