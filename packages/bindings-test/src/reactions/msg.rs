#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_DELETABLE_REGISTERED_REACTION_ID, TEST_EDITABLE_REGISTERED_REACTION_ID,
        TEST_POST_DELETABLE_REACTION_ID, TEST_REACTIONS_POST_ID, TEST_SUBSPACE,
    };
    use cosmwasm_std::Addr;
    use desmos_bindings::reactions::msg::ReactionsMsg;
    use desmos_bindings::reactions::types::{
        FreeTextValue, FreeTextValueParams, ReactionValue, RegisteredReactionValueParams,
    };
    use test_contract::msg::ExecuteMsg;

    #[test]
    fn test_add_reaction() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = ReactionsMsg::add_reaction(
            TEST_SUBSPACE,
            TEST_REACTIONS_POST_ID,
            ReactionValue::FreeText(FreeTextValue {
                text: "test".into(),
            }),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_remove_reaction() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = ReactionsMsg::remove_reaction(
            TEST_SUBSPACE,
            TEST_REACTIONS_POST_ID,
            TEST_POST_DELETABLE_REACTION_ID.into(),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_add_registered_reaction() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = ReactionsMsg::add_registered_reaction(
            TEST_SUBSPACE,
            "test".into(),
            "test".into(),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_edit_registered_reaction() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = ReactionsMsg::edit_registered_reaction(
            TEST_SUBSPACE,
            TEST_EDITABLE_REGISTERED_REACTION_ID.into(),
            "editable_code".into(),
            "editable_value".into(),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_remove_registered_reaction() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = ReactionsMsg::remove_registered_reaction(
            TEST_SUBSPACE,
            TEST_DELETABLE_REGISTERED_REACTION_ID.into(),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_set_reactions_params() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = ReactionsMsg::set_reactions_params(
            TEST_SUBSPACE,
            Some(RegisteredReactionValueParams { enabled: true }),
            Some(FreeTextValueParams {
                enabled: true,
                max_length: 5,
                reg_ex: "".to_string(),
            }),
            Addr::unchecked(&contract_address),
        );

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }
}
