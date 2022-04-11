#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DESMOS_HOME="$SCRIPT_DIR/.desmos"
KEYRING_PASS=pass1234
TEST_USER=manu
TEST_USER_MNEMONIC="math track fish reopen project latin radio spoon please table between install cheap smile deer glide desk license bench vapor chef sock list case"
TEST_USER_ADDRESS=desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3


desmos() {
	"$SCRIPT_DIR/desmos" --home="$DESMOS_HOME" "$@"
}

rm -r -f "$DESMOS_HOME"
desmos unsafe-reset-all
desmos init testchain --chain-id=testchain

(echo "$TEST_USER_MNEMONIC"; echo $KEYRING_PASS; echo $KEYRING_PASS) | desmos keys add "$TEST_USER" --recover --keyring-backend=file
echo $KEYRING_PASS | desmos add-genesis-account $TEST_USER 200000000000000stake --keyring-backend=file
echo $KEYRING_PASS | desmos gentx $TEST_USER 100000000000stake --amount 100000000000stake --chain-id=testchain --keyring-backend=file
desmos collect-gentxs
desmos start
