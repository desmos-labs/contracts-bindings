#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DESMOS_HOME="$SCRIPT_DIR/.desmos"
KEYRING_PASS=pass1234
# User 1 informations
USER1=user1
USER1_ADDRESS=desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3
USER1_MNEMONIC="math track fish reopen project latin radio spoon please table between install cheap smile deer glide desk license bench vapor chef sock list case"
# User 2 informations
USER2=user2
USER2_ADDRESS=desmos1ptvq7l4jt7n9sc3fky22mfvc6waf2jd8nuc0jv
USER2_MNEMONIC="invite steak example stage immense glad lawsuit shrimp script tennis oval symptom finish ride cactus camp butter local river pledge unfold kiwi vintage sorry"

# Background flags, will be set to true if the user pass the -b argument.
BACKGROUND=false

while getopts "b" arg; do
  case $arg in
    b)
      BACKGROUND=true
      ;;
  esac
done

desmos() {
	"$SCRIPT_DIR/desmos" --home="$DESMOS_HOME" "$@"
}

rm -r -f "$DESMOS_HOME"
desmos tendermint unsafe-reset-all
desmos init testchain --chain-id=testchain
# Add a default reason to the reports module params
jq '.app_state.reports.params.standard_reasons[0] |= . + {"id":"1","title":"Spam","description":"Spam user or content"}' "$DESMOS_HOME/config/genesis.json" > "$DESMOS_HOME/config/genesis-patched.json"
mv "$DESMOS_HOME/config/genesis-patched.json" "$DESMOS_HOME/config/genesis.json"

(echo "$USER1_MNEMONIC"; echo $KEYRING_PASS; echo $KEYRING_PASS) | desmos keys add "$USER1" --recover --keyring-backend=file
(echo "$USER2_MNEMONIC"; echo $KEYRING_PASS; echo $KEYRING_PASS) | desmos keys add "$USER2" --recover --keyring-backend=file
echo $KEYRING_PASS | desmos add-genesis-account $USER1 200000000000000stake --keyring-backend=file
echo $KEYRING_PASS | desmos add-genesis-account $USER2 200000000000000stake --keyring-backend=file
echo $KEYRING_PASS | desmos gentx $USER1 100000000000stake --amount 100000000000stake --chain-id=testchain --keyring-backend=file
desmos collect-gentxs


if [ $BACKGROUND = true ] ; then
  desmos start &> "$SCRIPT_DIR/desmos.log" &
else
  desmos start
fi

