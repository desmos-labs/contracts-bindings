#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DESMOS_HOME="$SCRIPT_DIR/.desmos"
KEYRING_PASS=pass1234
TEST_USER=manu
TEST_USER_MNEMONIC="math track fish reopen project latin radio spoon please table between install cheap smile deer glide desk license bench vapor chef sock list case"
TEST_USER_ADDRESS=desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3
SMART_CONTRACT="$SCRIPT_DIR/../artifacts/test_contract.wasm"


desmos() {
	"$SCRIPT_DIR/desmos" --home="$DESMOS_HOME" "$@"
}

# Upload the smart contract
echo $KEYRING_PASS | desmos tx wasm store "$SMART_CONTRACT" --from $TEST_USER --chain-id=testchain --keyring-backend=file -y --gas 2000000

# Wait a block time
echo "Contract deployed, waiting a block..."
sleep 8

# Initialize the contract
echo $KEYRING_PASS | desmos --keyring-backend=file tx wasm instantiate 1 "{}" \
  --from $TEST_USER --label "test-contract" --admin $TEST_USER_ADDRESS --chain-id=testchain -y
echo "Contract initialized"

# Wait block time
echo "Wait block time before first interaction"
sleep 8

# Print contract address
CONTRACT=$(desmos query wasm list-contract-by-code 1 --output json | jq -r '.contracts[-1]')
echo "Contract address $CONTRACT"

# Call the save profile command
#SAVE_PROFILE="{\"save_profile\": {\"dtag\": \"test_dtag\", \"nickname\": \"test-nick\", \"bio\": \"test-bio\", \"cover_picture\": \"https://i.imgur.com/X2aK5Bq.jpeg\", \"profile_picture\":\"https://i.imgur.com/X2aK5Bq.jpeg\"}}"
#echo $KEYRING_PASS | desmos tx wasm execute "$CONTRACT" "$SAVE_PROFILE" --from $TEST_USER --keyring-backend=file --chain-id=testchain
