#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DESMOS_HOME="$SCRIPT_DIR/.desmos"
KEYRING_PASS=pass1234

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
``
# Create a relationships between user1 and user2
echo "Create relationship between user1 and user2"
echo $KEYRING_PASS | desmos tx relationships create-relationship $USER2_ADDRESS 1 --from $USER1 \
  --chain-id=testchain --keyring-backend=file -b=block -y

# Create a block from user2 to user1
echo "Create block from user2 to user1"
echo $KEYRING_PASS | desmos tx relationships block $USER1_ADDRESS 1 --from $USER2 \
  --chain-id=testchain --keyring-backend=file -b=block -y