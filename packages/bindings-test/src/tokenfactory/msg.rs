#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_CREATION_DENOM_FEES_AMOUNT, TEST_CREATION_DENOM_FEES_DENOM};
    use crate::subspaces::create_test_subspace;
    use crate::tokenfactory::get_denom;

    use cosmwasm_std::{Addr, Coin};
    use desmos_bindings::cosmos_types::{DenomUnit, Metadata};
    use desmos_bindings::tokenfactory::msg::TokenfactoryMsg;

    #[test]
    fn test_create_denom() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Setup subspace and deposit creation fees
        let subspace = create_test_subspace(&contract_address);
        desmos_cli
            .send_tokens(
                &subspace.treasury,
                Coin::new(
                    TEST_CREATION_DENOM_FEES_AMOUNT,
                    TEST_CREATION_DENOM_FEES_DENOM,
                ),
            )
            .assert_success();

        let msg = TokenfactoryMsg::create_denom(
            subspace.id,
            Addr::unchecked(&contract_address),
            "test_create",
        );
        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_mint_then_burn() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subdenom = "test_mint_burn";

        // Setup subspace and deposit creation fees
        let subspace = create_test_subspace(&contract_address);
        desmos_cli
            .send_tokens(
                &subspace.treasury,
                Coin::new(
                    TEST_CREATION_DENOM_FEES_AMOUNT,
                    TEST_CREATION_DENOM_FEES_DENOM,
                ),
            )
            .assert_success();

        let create_denom_msg = TokenfactoryMsg::create_denom(
            subspace.id,
            Addr::unchecked(&contract_address),
            subdenom,
        );

        let mint_msg = TokenfactoryMsg::mint(
            subspace.id,
            Addr::unchecked(&contract_address),
            Coin::new(100, get_denom(&subspace, subdenom)),
        );

        let burn_msg = TokenfactoryMsg::burn(
            subspace.id,
            Addr::unchecked(&contract_address),
            Coin::new(100, get_denom(&subspace, subdenom)),
        );

        desmos_cli
            .execute_contract(
                &contract_address,
                vec![create_denom_msg.into(), mint_msg.into(), burn_msg.into()],
            )
            .assert_success();
    }

    #[test]
    fn test_set_metadata() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subdenom = "test_set_metadata";

        // Setup subspace and deposit creation fees
        let subspace = create_test_subspace(&contract_address);
        desmos_cli
            .send_tokens(
                &subspace.treasury,
                Coin::new(
                    TEST_CREATION_DENOM_FEES_AMOUNT,
                    TEST_CREATION_DENOM_FEES_DENOM,
                ),
            )
            .assert_success();

        let create_denom_msg = TokenfactoryMsg::create_denom(
            subspace.id,
            Addr::unchecked(&contract_address),
            subdenom,
        );

        let set_metadata_msg = TokenfactoryMsg::set_denom_metadata(
            subspace.id,
            Addr::unchecked(&contract_address),
            Metadata {
                name: "test".into(),
                description: "test".into(),
                symbol: "test".into(),
                denom_units: vec![
                    DenomUnit {
                        denom: get_denom(&subspace, subdenom),
                        exponent: 0,
                        aliases: vec![],
                    },
                    DenomUnit {
                        denom: "test".into(),
                        exponent: 6,
                        aliases: vec!["test".into()],
                    },
                ],
                base: get_denom(&subspace, subdenom),
                display: "test".into(),
                uri: "".into(),
                uri_hash: "".into(),
            },
        );

        desmos_cli
            .execute_contract(
                &contract_address,
                vec![create_denom_msg.into(), set_metadata_msg.into()],
            )
            .assert_success();
    }
}
