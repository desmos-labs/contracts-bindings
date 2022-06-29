#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_SUBSPACE, TEST_REACTIONS_POST_ID, TEST_POST_DELETABLE_REACTION_ID,
        TEST_EDITABLE_REGISTERED_REACTION_ID, TEST_DELETABLE_REGISTERED_REACTION_ID,
    };
    use cosmwasm_std::Addr;
    use desmos_bindings::reactions::msg::ReactionsMsg;
    use desmos_bindings::reactions::models::{ReactionValue, RegisteredReactionValueParams, FreeTextValueParams};
    use test_contract::msg::ExecuteMsg;

    #[test]
    fn test_add_reaction() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = ReactionsMsg::AddReaction { 
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_REACTIONS_POST_ID,
            value: ReactionValue::FreeText{ text: "test".to_string() }.into(),
            user: Addr::unchecked(&contract_address),
        };

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

        let msg = ReactionsMsg::RemoveReaction { 
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_REACTIONS_POST_ID,
            reaction_id: TEST_POST_DELETABLE_REACTION_ID,
            user: Addr::unchecked(&contract_address),
        };

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

        let msg = ReactionsMsg::AddRegisteredReaction { 
            subspace_id: TEST_SUBSPACE,
            shorthand_code: "test".to_string(),
            display_value: "test".to_string(),
            user: Addr::unchecked(&contract_address),
        };

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

        let msg = ReactionsMsg::EditRegisteredReaction { 
            subspace_id: TEST_SUBSPACE,
            registered_reaction_id: TEST_EDITABLE_REGISTERED_REACTION_ID,
            shorthand_code: "editable_code".to_string(),
            display_value: "editable_value".to_string(),
            user: Addr::unchecked(&contract_address),
        };

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

        let msg = ReactionsMsg::RemoveRegisteredReaction { 
            subspace_id: TEST_SUBSPACE,
            registered_reaction_id: TEST_DELETABLE_REGISTERED_REACTION_ID,
            user: Addr::unchecked(&contract_address),
        };

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

        let msg = ReactionsMsg::SetReactionsParams { 
            subspace_id: TEST_SUBSPACE,
            registered_reaction: RegisteredReactionValueParams{ enabled: true },
            free_text: FreeTextValueParams{ enabled: true, max_length: 5, reg_ex: "".to_string() },
            user: Addr::unchecked(&contract_address),
        };

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