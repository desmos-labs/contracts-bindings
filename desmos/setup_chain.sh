#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DESMOS_HOME="$SCRIPT_DIR/.desmos"
KEYRING_PASS=pass1234
# Smart contract dir
SMART_CONTRACT="$SCRIPT_DIR/../artifacts/test_contract.wasm"
# User 1 informations
USER1=user1
USER1_MNEMONIC="math track fish reopen project latin radio spoon please table between install cheap smile deer glide desk license bench vapor chef sock list case"
USER1_ADDRESS=desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3
# User 2 informations
USER2=user2
USER2_MNEMONIC="invite steak example stage immense glad lawsuit shrimp script tennis oval symptom finish ride cactus camp butter local river pledge unfold kiwi vintage sorry"
USER2_ADDRESS=desmos1ptvq7l4jt7n9sc3fky22mfvc6waf2jd8nuc0jv


desmos() {
	"$SCRIPT_DIR/desmos" --home="$DESMOS_HOME" "$@"
}

# Force the script to exit at the first error
set -e

# Upload the smart contract
echo "Uploading contract..."
echo $KEYRING_PASS | desmos tx wasm store "$SMART_CONTRACT" \
  --from $USER1 --chain-id=testchain --keyring-backend=file -y --gas 2000000 \
  -b=block

# Initialize the contract
echo "Initializing contract..."
echo $KEYRING_PASS | desmos --keyring-backend=file tx wasm instantiate 1 "{}" \
  --from $USER1 --label "test-contract" --admin $USER1_ADDRESS --chain-id=testchain -b=block -y
echo "Contract initialized"

# Print contract address
CONTRACT=$(desmos query wasm list-contract-by-code 1 --output json | jq -r '.contracts[-1]')
echo "Contract address $CONTRACT"

# Create user1 profile
echo "Create user1 profile"
echo $KEYRING_PASS | desmos tx profiles save \
  --from $USER1 --dtag user1 --home=$PWD/.desmos --chain-id=testchain --keyring-backend=file -b=block -y

# Create user2 profile
echo "Create user2 profile"
echo $KEYRING_PASS | desmos tx profiles save \
  --from $USER2 --dtag user2 --home=$PWD/.desmos --chain-id=testchain --keyring-backend=file -b=block -y

# Create test subspace
echo "Create test subspace"
echo $KEYRING_PASS | desmos tx subspaces create "test subspace" \
  --description "Test subspace" \
  --treasury $USER1_ADDRESS \
  --owner $USER1_ADDRESS \
  --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y
