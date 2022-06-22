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
# Chain link data files
COSMOS_CHAIN_LINK_DATA="$SCRIPT_DIR/cosmos-chain-link-data.json"
OSMOSIS_CHAIN_LINK_DATA="$SCRIPT_DIR/osmosis-chain-link-data.json"


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
echo $KEYRING_PASS | desmos tx profiles save --from $USER1 \
  --dtag user1 --nickname user1 --bio "user1 bio" \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Link a cosmos address
echo "Link cosmos address"
echo $KEYRING_PASS | desmos tx profiles link-chain "$COSMOS_CHAIN_LINK_DATA" --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Link a osmosis address
echo "Link osmosis address"
echo $KEYRING_PASS | desmos tx profiles link-chain "$OSMOSIS_CHAIN_LINK_DATA" --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Create user2 profile
echo "Create user2 profile"
echo $KEYRING_PASS | desmos tx profiles save --from $USER2 \
  --dtag user2 --nickname user2 --bio "user2 bio" \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Create a dtag transfer request from user2 to user1
echo $KEYRING_PASS | desmos tx profiles request-dtag-transfer $USER1_ADDRESS --from $USER2 \
  --keyring-backend=file --chain-id=testchain -b=block -y

# Create test subspace owned by the smart contract
MSG="{\"desmos_messages\":{\"msgs\":[{\"custom\":{\"route\":\"subspaces\",\"msg_data\":{\"create_subspace\":{\"name\":\"Test subspace\",\"description\":\"\",\"treasury\":\"$CONTRACT\",\"owner\":\"$CONTRACT\",\"creator\":\"$CONTRACT\"}}}}]}}"
echo "Create test subspace"
echo $KEYRING_PASS | desmos tx wasm execute "$CONTRACT" "$MSG" \
  --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Create a test user group owned by the smart contract
MSG="{\"desmos_messages\":{\"msgs\":[{\"custom\":{\"route\":\"subspaces\",\"msg_data\":{\"create_user_group\":{\"subspace_id\":\"1\",\"section_id\":null,\"name\":\"Test user group\",\"description\":null,\"default_permissions\":[\"EDIT_SUBSPACE\"],\"creator\":\"$CONTRACT\"}}}}]}}"
echo "Create test user group"
echo $KEYRING_PASS | desmos tx wasm execute "$CONTRACT" "$MSG" \
  --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Add user1 to the user group owned by the smart contract
MSG="{\"desmos_messages\":{\"msgs\":[{\"custom\":{\"route\":\"subspaces\",\"msg_data\":{\"add_user_to_user_group\":{\"subspace_id\":\"1\",\"group_id\":1,\"user\":\"$USER1_ADDRESS\",\"signer\":\"$CONTRACT\"}}}}]}}"
echo "Add user1 to the user group"
echo $KEYRING_PASS | desmos tx wasm execute "$CONTRACT" "$MSG" \
  --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Set user1 permissions inside the test subspace
MSG="{\"desmos_messages\":{\"msgs\":[{\"custom\":{\"route\":\"subspaces\",\"msg_data\":{\"set_user_permissions\":{\"subspace_id\":\"1\",\"section_id\":0,\"user\":\"$USER1_ADDRESS\",\"permissions\":[\"EDIT_SUBSPACE\",\"DELETE_SUBSPACE\",\"MANAGE_GROUPS\"],\"signer\":\"$CONTRACT\"}}}}]}}"
echo "Set user1 permissions inside the test subspace"
echo $KEYRING_PASS | desmos tx wasm execute "$CONTRACT" "$MSG" \
  --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Create a relationships between user1 and user2
echo "Create relationship between user1 and user2"
echo $KEYRING_PASS | desmos tx relationships create-relationship $USER2_ADDRESS 1 --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Create a block from user2 to user1
echo "Create block from user2 to user1"
echo $KEYRING_PASS | desmos tx relationships block $USER1_ADDRESS 1 --from $USER2 \
  --chain-id=testchain --keyring-backend=file -b=block -y
