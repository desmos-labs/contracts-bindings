#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{USER1_ADDRESS, USER1_KEY, USER2_ADDRESS, USER2_KEY};
    use cosmwasm_std::{Addr, Binary};
    use desmos_bindings::profiles::msg::ProfilesMsg;
    use desmos_bindings::profiles::types::{
        AddressData, Bech32Address, ChainConfig, Proof, SignatureValueType, SingleSignature,
    };
    use desmos_bindings::types::Secp256k1PublicKey;
    use test_contract::msg::ExecuteMsg;

    fn build_save_profile_msg(contract_address: &str) -> ExecuteMsg {
        ExecuteMsg::DesmosMessages {
            msgs: vec![ProfilesMsg::save_profile(
                Some("test_profile"),
                Some("contract_nick"),
                Some("test_bio"),
                Some("https://i.imgur.com/X2aK5Bq.jpeg"),
                Some("https://i.imgur.com/X2aK5Bq.jpeg"),
                Addr::unchecked(contract_address),
            )
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

        let delete_profile_msg =
            ProfilesMsg::delete_profile(Addr::unchecked(&contract_address));

        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![delete_profile_msg.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    pub fn test_request_dtag_transfer() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let dtag_transfer_request = ProfilesMsg::request_dtag_transfer(
            Addr::unchecked(&contract_address),
            Addr::unchecked(USER2_ADDRESS),
        );

        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![dtag_transfer_request.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let cancel_dtag_transfer_request = ProfilesMsg::cancel_dtag_transfer_request(
            Addr::unchecked(USER2_ADDRESS),
            Addr::unchecked(&contract_address),
        );

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
        let save_profile = ProfilesMsg::save_profile(
            Some("test_accept_dtag_transfer"),
            Some("contract_nick"),
            Some("test_bio"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Addr::unchecked(&contract_address),
        );

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
        let accept_dtag_transfer_request = ProfilesMsg::accept_dtag_transfer_request(
            "user2",
            Addr::unchecked(USER2_ADDRESS),
            Addr::unchecked(&contract_address),
        );

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
        let save_profile = ProfilesMsg::save_profile(
            Some("test_refuse_dtag_transfer"),
            Some("contract_nick"),
            Some("test_bio"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Addr::unchecked(&contract_address),
        );

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

        // Prepare the RefuseDtagTransferRequest msg for the smart contract
        let refuse_dtag_transfer_request = ProfilesMsg::refuse_dtag_transfer_request(
            Addr::unchecked(USER1_ADDRESS),
            Addr::unchecked(&contract_address),
        );

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
        let save_profile = ProfilesMsg::save_profile(
            Some("test_link_chain_account"),
            Some("contract_nick"),
            Some("test_bio"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Addr::unchecked(&contract_address),
        );

        // Prepare the LinkChainAccount msg for the smart contract
        let link_chain_account = ProfilesMsg::link_chain_account(
            AddressData::Bech32Address(Bech32Address{
            value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
            prefix:"cosmos".into()
            }),
            Proof{
                pub_key: Some(Secp256k1PublicKey{ key: Binary::from_base64("A6p7imM9YY/uFgZFV/ZiNQ45Ki2xbyR4zjG//BFzkVtY").unwrap().into() }.into()),
                signature: Some(SingleSignature{
                    value_type: SignatureValueType::Raw.into(),
                    signature: Binary::from_base64("tNuudGWFCKhjzN1twCYMkZHWYNxlCcXPeD7PL1rGiO0oUjhYglADFT6mjecKiHQLyW4COeRpvKSnGByQkCZZkA==").unwrap().into(),
                }.into()),
                plain_text: "6465736d6f733134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736c6d66666c76".into(),
            },
            ChainConfig { name: "cosmos".into() },
            Addr::unchecked(&contract_address),
        );

        // Prepare the UnlinkChainAccount msg for the smart contract
        let unlink_chain_account = ProfilesMsg::unlink_chain_account(
            Addr::unchecked(&contract_address),
            "cosmos".into(),
            "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
        );

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
        let save_profile = ProfilesMsg::save_profile(
            Some("test_default_external_address"),
            Some("contract_nick"),
            Some("test_bio"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Some("https://i.imgur.com/X2aK5Bq.jpeg"),
            Addr::unchecked(&contract_address),
        );

        // Prepare the LinkChainAccount msg of the first address for the smart contract
        let link_first_chain_account = ProfilesMsg::link_chain_account(
            AddressData::Bech32Address(Bech32Address{
            value: "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
            prefix:"cosmos".into()
            }),
            Proof{
                pub_key: Some(Secp256k1PublicKey{ key: Binary::from_base64("A6p7imM9YY/uFgZFV/ZiNQ45Ki2xbyR4zjG//BFzkVtY").unwrap().into() }.into()),
                signature: Some(SingleSignature{
                    value_type: SignatureValueType::Raw.into(),
                    signature: Binary::from_base64("tNuudGWFCKhjzN1twCYMkZHWYNxlCcXPeD7PL1rGiO0oUjhYglADFT6mjecKiHQLyW4COeRpvKSnGByQkCZZkA==").unwrap().into(),
                }.into()),
                plain_text: "6465736d6f733134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736c6d66666c76".into(),
            },
            ChainConfig { name: "cosmos".into() },
            Addr::unchecked(&contract_address),
        );

        // Prepare the LinkChainAccount msg of the second address for the smart contract
        let link_second_chain_account = ProfilesMsg::link_chain_account(
            AddressData::Bech32Address(Bech32Address{
            value: "cosmos13n9wek2ktpxhpgfrd39zlaqaeahxuyusxrsfvn".into(),
            prefix:"cosmos".into()
            }),
            Proof{
                pub_key: Some(Secp256k1PublicKey{ key: Binary::from_base64("AqYZhHKaeBcrYktZEvor/SUDlHCkv5JBplaG2vc2bvfS").unwrap().into() }.into()),
                signature: Some(SingleSignature{
                    value_type: SignatureValueType::Raw.into(),
                    signature: Binary::from_base64("gLIWKbyZ8nUtCVvr8TfPGDYU1rybwPDi6neMuEjfvkwNXJVuNcmthqVeuvxEln7K15PIEPUGTMTV/kU0n3iGPw==").unwrap().into(),
                }.into()),
                plain_text: "6465736d6f733134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736c6d66666c76".into(),
            },
            ChainConfig { name: "cosmos".into() },
            Addr::unchecked(&contract_address),
        );

        // Prepare the SetDefaultExternalAddress msg
        let set_default_external_address = ProfilesMsg::set_default_external_address(
            "cosmos",
            "cosmos13n9wek2ktpxhpgfrd39zlaqaeahxuyusxrsfvn",
            Addr::unchecked(&contract_address),
        );

        // Prepare the UnlinkChainAccount msg for first chain account
        let unlink_first_chain_account = ProfilesMsg::unlink_chain_account(
            Addr::unchecked(&contract_address),
            "cosmos".into(),
            "cosmos1wrx0kayjzuf27gaaqult0z576y0xggq00mrc2r".into(),
        );

        // Prepare the UnlinkChainAccount msg for second chain account
        let unlink_second_chain_account = ProfilesMsg::unlink_chain_account(
            Addr::unchecked(&contract_address),
            "cosmos".into(),
            "cosmos13n9wek2ktpxhpgfrd39zlaqaeahxuyusxrsfvn".into(),
        );

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
