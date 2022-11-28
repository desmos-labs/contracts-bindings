#!/usr/bin/env bash

KEYRING_PASS=pass1234
# User 1 informations
USER1=user1
USER1_ADDRESS=desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3
USER1_MNEMONIC="math track fish reopen project latin radio spoon please table between install cheap smile deer glide desk license bench vapor chef sock list case"
# User 2 informations
USER2=user2
USER2_ADDRESS=desmos1ptvq7l4jt7n9sc3fky22mfvc6waf2jd8nuc0jv
USER2_MNEMONIC="invite steak example stage immense glad lawsuit shrimp script tennis oval symptom finish ride cactus camp butter local river pledge unfold kiwi vintage sorry"

echo "Desmos home $DESMOS_HOME"
echo "Desmos bin $DESMOS_BIN1"

desmos() {
	$DESMOS_BIN --home="$DESMOS_HOME" "$@"
}

# Set block time to 500 milliseconds
sed -i -e 's/timeout_commit = "5s"/timeout_commit = "500ms"/g' "$DESMOS_HOME/config/config.toml"

# Add users to keyring
(echo "$USER1_MNEMONIC"; echo $KEYRING_PASS; echo $KEYRING_PASS) | desmos keys add "$USER1" --recover --keyring-backend=file
(echo "$USER2_MNEMONIC"; echo $KEYRING_PASS; echo $KEYRING_PASS) | desmos keys add "$USER2" --recover --keyring-backend=file


