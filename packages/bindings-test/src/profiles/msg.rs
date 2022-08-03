#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{USER1_ADDRESS, USER1_KEY, USER2_ADDRESS, USER2_KEY};
    use cosmwasm_std::{Addr, Binary};
    use desmos_bindings::profiles::models_chain_links::{
        Address, ChainConfig, Proof, Signature, SignatureValueType,
    };
    use desmos_bindings::profiles::msg::ProfilesMsg;
    use desmos_bindings::profiles::msg::ProfilesMsg::{DeleteProfile, SaveProfile};
    use desmos_bindings::types::PubKey;
    use test_contract::msg::ExecuteMsg;

    fn build_save_profile_msg(contract_address: &str) -> ExecuteMsg {
        ExecuteMsg::DesmosMessages {
            msgs: vec![SaveProfile {
                dtag: "test_profile".to_string(),
                nickname: "contract_nick".to_string(),
                bio: "test_bio".to_string(),
                profile_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
                cover_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
                creator: Addr::unchecked(contract_address),
            }
            .into()],
        }
    }

    #[test]
    fn test_create_profile() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let save_profile_msg = build_save_profile_msg(&contract_address);

        desmos_cli
            .wasm_execute(&contract_address, &save_profile_msg)
            .assert_success();
    }

    #[test]
    fn test_delete_profile() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let save_profile_msg = build_save_profile_msg(&contract_address);

        desmos_cli
            .wasm_execute(&contract_address, &save_profile_msg)
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
            receiver: Addr::unchecked(USER2_ADDRESS),
            sender: Addr::unchecked(&contract_address),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![dtag_transfer_request.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let cancel_dtag_transfer_request = ProfilesMsg::CancelDtagTransferRequest {
            receiver: Addr::unchecked(USER2_ADDRESS),
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
                &format!("--from={}", USER2_KEY),
            ])
            .assert_success();

        // Prepare the AcceptDtagTransferRequest msg for the smart contract
        let accept_dtag_transfer_request = ProfilesMsg::AcceptDtagTransferRequest {
            new_dtag: "user2".to_string(),
            receiver: Addr::unchecked(&contract_address),
            sender: Addr::unchecked(USER2_ADDRESS),
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
    fn test_link_unlink_chain_account() {
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
            chain_address: Address {
                proto_type: "/desmos.profiles.v3.Bech32Address".to_string(),
                value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".to_string(),
                prefix: Some("cosmos".to_string()),
            },
            proof: Proof {
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: Binary::from_base64("A6p7imM9YY/uFgZFV/ZiNQ45Ki2xbyR4zjG//BFzkVtY").unwrap(),
                },
                signature: Signature {
                    proto_type: "/desmos.profiles.v3.SingleSignature".to_string(),
                    value_type: SignatureValueType::Raw,
                    signature: Binary::from_base64("tNuudGWFCKhjzN1twCYMkZHWYNxlCcXPeD7PL1rGiO0oUjhYglADFT6mjecKiHQLyW4COeRpvKSnGByQkCZZkA==").unwrap(),
                },
                plain_text: "6465736d6f733134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736c6d66666c76".to_string(),
            },
            chain_config: ChainConfig { name: "cosmos".to_string() },
            signer: Addr::unchecked(&contract_address),
        };

        // Prepare the UnlinkChainAccount msg for the smart contract
        let unlink_chain_account = ProfilesMsg::UnlinkChainAccount {
            owner: Addr::unchecked(&contract_address),
            chain_name: "cosmos".to_string(),
            target: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".to_string(),
        };

        // Wrap the message into the smart contract message
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![
                save_profile.into(),
                link_chain_account.into(),
                unlink_chain_account.into(),
            ],
        };

        // Execute the tx
        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_set_default_external_address() {
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

        // Prepare the LinkChainAccount msg of the first address for the smart contract
        let link_first_chain_account = ProfilesMsg::LinkChainAccount {
            chain_address: Address {
                proto_type: "/desmos.profiles.v3.Bech32Address".to_string(),
                value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".to_string(),
                prefix: Some("cosmos".to_string()),
            },
            proof: Proof {
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: Binary::from_base64("A6p7imM9YY/uFgZFV/ZiNQ45Ki2xbyR4zjG//BFzkVtY").unwrap(),
                },
                signature: Signature {
                    proto_type: "/desmos.profiles.v3.SingleSignature".to_string(),
                    value_type: SignatureValueType::Raw,
                    signature: Binary::from_base64("tNuudGWFCKhjzN1twCYMkZHWYNxlCcXPeD7PL1rGiO0oUjhYglADFT6mjecKiHQLyW4COeRpvKSnGByQkCZZkA==").unwrap(),
                },
                plain_text: "6465736d6f733134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736c6d66666c76".to_string(),
            },
            chain_config: ChainConfig { name: "cosmos".to_string() },
            signer: Addr::unchecked(&contract_address),
        };

        // Prepare the LinkChainAccount msg of the second address for the smart contract
        let link_second_chain_account = ProfilesMsg::LinkChainAccount {
            chain_address: Address {
                proto_type: "/desmos.profiles.v3.Bech32Address".to_string(),
                value: "cosmos13n9wek2ktpxhpgfrd39zlaqaeahxuyusxrsfvn".to_string(),
                prefix: Some("cosmos".to_string()),
            },
            proof: Proof {
                pub_key: PubKey {
                    proto_type: "/cosmos.crypto.secp256k1.PubKey".to_string(),
                    key: Binary::from_base64("AqYZhHKaeBcrYktZEvor/SUDlHCkv5JBplaG2vc2bvfS").unwrap(),
                },
                signature: Signature {
                    proto_type: "/desmos.profiles.v3.SingleSignature".to_string(),
                    value_type: SignatureValueType::Raw,
                    signature: Binary::from_base64("gLIWKbyZ8nUtCVvr8TfPGDYU1rybwPDi6neMuEjfvkwNXJVuNcmthqVeuvxEln7K15PIEPUGTMTV/kU0n3iGPw==").unwrap(),
                },
                plain_text: "6465736d6f733134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736c6d66666c76".to_string(),
            },
            chain_config: ChainConfig { name: "cosmos".to_string() },
            signer: Addr::unchecked(&contract_address),
        };

        // Prepare the SetDefaultExternalAddress msg
        let set_default_external_address = ProfilesMsg::set_default_external_address(
            "cosmos",
            "cosmos13n9wek2ktpxhpgfrd39zlaqaeahxuyusxrsfvn",
            Addr::unchecked(&contract_address),
        );

        // Prepare the UnlinkChainAccount msg for first chain account
        let unlink_first_chain_account = ProfilesMsg::UnlinkChainAccount {
            owner: Addr::unchecked(&contract_address),
            chain_name: "cosmos".to_string(),
            target: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".to_string(),
        };

        // Prepare the UnlinkChainAccount msg for second chain account
        let unlink_second_chain_account = ProfilesMsg::UnlinkChainAccount {
            owner: Addr::unchecked(&contract_address),
            chain_name: "cosmos".to_string(),
            target: "cosmos13n9wek2ktpxhpgfrd39zlaqaeahxuyusxrsfvn".to_string(),
        };

        // Wrap the message into the smart contract message
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![
                save_profile.into(),
                link_first_chain_account.into(),
                link_second_chain_account.into(),
                set_default_external_address.into(),
                unlink_first_chain_account.into(),
                unlink_second_chain_account.into(),
            ],
        };

        // Execute the tx
        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
