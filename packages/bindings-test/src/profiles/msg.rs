#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{USER1_ADDRESS, USER1_KEY};
    use cosmwasm_std::{Addr, Binary};
    use desmos_bindings::profiles::models_chain_links::{
        Address, ChainConfig, Proof, SignMode, Signature,
    };
    use desmos_bindings::profiles::msg::ProfilesMsg;
    use desmos_bindings::profiles::msg::ProfilesMsg::{DeleteProfile, SaveProfile};
    use desmos_bindings::types::PubKey;
    use test_contract::msg::ExecuteMsg;

    #[test]
    fn test_create_delete_profile() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let save_profile = SaveProfile {
            dtag: "test_create_delete_profile".to_string(),
            nickname: "contract_nick".to_string(),
            bio: "test_bio".to_string(),
            profile_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            cover_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            creator: Addr::unchecked(contract_address.clone()),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![save_profile.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let delete_profile = DeleteProfile {
            creator: Addr::unchecked(contract_address.clone()),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![delete_profile.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    pub fn test_request_dtag_transfer() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let dtag_transfer_request = ProfilesMsg::RequestDtagTransfer {
            receiver: Addr::unchecked(USER1_ADDRESS),
            sender: Addr::unchecked(&contract_address),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![dtag_transfer_request.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let cancel_dtag_transfer_request = ProfilesMsg::CancelDtagTransferRequest {
            receiver: Addr::unchecked(USER1_ADDRESS),
            sender: Addr::unchecked(&contract_address),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![cancel_dtag_transfer_request.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_accept_dtag_transfer() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Create a profile for the smart contract
        let save_profile = SaveProfile {
            dtag: "test_accept_dtag_transfer".to_string(),
            nickname: "contract_nick".to_string(),
            bio: "test_bio".to_string(),
            profile_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            cover_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            creator: Addr::unchecked(contract_address.clone()),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![save_profile.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        // Request a dtag transfer from the user1 to the smart contract
        desmos_cli
            .execute_tx([
                "profiles",
                "request-dtag-transfer",
                &contract_address,
                &format!("--from={}", USER1_KEY),
            ])
            .assert_success();

        // Prepare the AcceptDtagTransferRequest msg for the smart contract
        let accept_dtag_transfer_request = ProfilesMsg::AcceptDtagTransferRequest {
            new_dtag: "user1".to_string(),
            receiver: Addr::unchecked(&contract_address),
            sender: Addr::unchecked(USER1_ADDRESS),
        };

        // Wrap the message into the smart contract message
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![accept_dtag_transfer_request.into()],
        };

        // Execute the tx
        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_refuse_dtag_transfer_request() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Create a profile for the smart contract
        let save_profile = SaveProfile {
            dtag: "test_refuse_dtag_transfer".to_string(),
            nickname: "contract_nick".to_string(),
            bio: "test_bio".to_string(),
            profile_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            cover_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            creator: Addr::unchecked(contract_address.clone()),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![save_profile.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        // Request a dtag transfer from the user1 to the smart contract
        desmos_cli
            .execute_tx([
                "profiles",
                "request-dtag-transfer",
                &contract_address,
                &format!("--from={}", USER1_KEY),
            ])
            .assert_success();

        // Prepare the AcceptDtagTransferRequest msg for the smart contract
        let refuse_dtag_transfer_request = ProfilesMsg::RefuseDtagTransferRequest {
            sender: Addr::unchecked(USER1_ADDRESS),
            receiver: Addr::unchecked(&contract_address),
        };

        // Wrap the message into the smart contract message
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![refuse_dtag_transfer_request.into()],
        };

        // Execute the tx
        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_link_chain_account() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Create a profile for the smart contract
        let save_profile = SaveProfile {
            dtag: "test_link_chain_account".to_string(),
            nickname: "contract_nick".to_string(),
            bio: "test_bio".to_string(),
            profile_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            cover_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            creator: Addr::unchecked(contract_address.clone()),
        };

        // Prepare the LinkChainAccount msg for the smart contract
        let link_chain_account = ProfilesMsg::LinkChainAccount {
            chain_address: Address::Bech32 {
                value: "cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string(),
                prefix: "cosmos".to_string(),
            },
            proof: Proof {
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: "AyRUhKXAY6zOCjjFkPN78Q29sBKHjUx4VSZQ4HXh66IM".to_string(),
                },
                signature: Signature::SingleSignatureData {
                    mode: SignMode::Direct,
                    signature: Binary::from_base64("C7xppu4C4S3dgeC9TVqhyGN1hbMnMbnmWgXQI2WE8t0oHIHhDTqXyZgzhNNYiBO7ulno3G8EXO3Ep5KMFngyFg").unwrap(),
                },
                plain_text: "636f736d6f733138786e6d6c7a71727172367a74353236706e637a786536357a6b33663478676d6e6470786e32".to_string(),
            },
            chain_config: ChainConfig { name: "cosmos".to_string() },
            signer: Addr::unchecked(&contract_address),
        };

        // Wrap the message into the smart contract message
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![save_profile.into(), link_chain_account.into()],
        };

        // Execute the tx
        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
