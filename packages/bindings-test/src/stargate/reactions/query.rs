#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_EDITABLE_REGISTERED_REACTION_ID, TEST_POST_FREE_TEXT_REACTION_ID,
        TEST_POST_REGISTERED_REACTION_ID, TEST_REACTIONS_POST_ID, TEST_SUBSPACE,
    };
    use desmos_bindings::stargate::reactions::types::{
        FreeTextValue, FreeTextValueParams, Reaction, ReactionValue, RegisteredReaction,
        RegisteredReactionValue, RegisteredReactionValueParams, SubspaceReactionsParams,
    };
    use desmos_bindings::stargate::reactions::types::{
        QueryReactionRequest, QueryReactionResponse, QueryReactionsParamsRequest,
        QueryReactionsParamsResponse, QueryReactionsRequest, QueryReactionsResponse,
        QueryRegisteredReactionRequest, QueryRegisteredReactionResponse,
        QueryRegisteredReactionsRequest, QueryRegisteredReactionsResponse,
    };
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_post_reactions() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryReactionsRequest {
                subspace_id: TEST_SUBSPACE.into(),
                post_id: TEST_REACTIONS_POST_ID.into(),
                user: "".into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryReactionsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();
        let reaction = result.reactions.first().unwrap();

        let expected = Reaction {
            subspace_id: TEST_SUBSPACE.into(),
            post_id: TEST_REACTIONS_POST_ID.into(),
            id: TEST_POST_REGISTERED_REACTION_ID,
            value: Some(
                ReactionValue::Registered(RegisteredReactionValue {
                    registered_reaction_id: TEST_EDITABLE_REGISTERED_REACTION_ID,
                })
                .into(),
            ),
            author: contract_address.into(),
        };
        assert_eq!(&expected, reaction);
    }

    #[test]
    fn test_query_post_registered_reaction() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryReactionRequest {
                subspace_id: TEST_SUBSPACE.into(),
                post_id: TEST_REACTIONS_POST_ID.into(),
                reaction_id: TEST_POST_REGISTERED_REACTION_ID,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryReactionResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = Reaction {
            subspace_id: TEST_SUBSPACE.into(),
            post_id: TEST_REACTIONS_POST_ID.into(),
            id: TEST_POST_REGISTERED_REACTION_ID,
            value: Some(
                ReactionValue::Registered(RegisteredReactionValue {
                    registered_reaction_id: TEST_EDITABLE_REGISTERED_REACTION_ID,
                })
                .into(),
            ),
            author: contract_address.into(),
        };
        assert_eq!(expected, result.reaction.unwrap());
    }

    #[test]
    fn test_query_post_free_text_reaction() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryReactionRequest {
                subspace_id: TEST_SUBSPACE.into(),
                post_id: TEST_REACTIONS_POST_ID.into(),
                reaction_id: TEST_POST_FREE_TEXT_REACTION_ID,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryReactionResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = Reaction {
            subspace_id: TEST_SUBSPACE.into(),
            post_id: TEST_REACTIONS_POST_ID.into(),
            id: TEST_POST_FREE_TEXT_REACTION_ID,
            value: Some(
                FreeTextValue {
                    text: "test".into(),
                }
                .into(),
            ),
            author: contract_address.into(),
        };
        assert_eq!(expected, result.reaction.unwrap());
    }

    #[test]
    fn test_query_registered_reactions() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryRegisteredReactionsRequest {
                subspace_id: TEST_SUBSPACE.into(),
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryRegisteredReactionsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();
        let registered_reaction = result.registered_reactions.first().unwrap();

        let expected = RegisteredReaction {
            subspace_id: TEST_SUBSPACE.into(),
            id: 1,
            shorthand_code: "editable_code".into(),
            display_value: "editable_value".into(),
        };
        assert_eq!(&expected, registered_reaction);
    }

    #[test]
    fn test_query_registered_reaction() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryRegisteredReactionRequest {
                subspace_id: TEST_SUBSPACE.into(),
                reaction_id: 1,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryRegisteredReactionResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = RegisteredReaction {
            subspace_id: TEST_SUBSPACE.into(),
            id: 1,
            shorthand_code: "editable_code".into(),
            display_value: "editable_value".into(),
        };
        assert_eq!(expected, result.registered_reaction.unwrap());
    }

    #[test]
    fn test_query_reactions_params() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: QueryReactionsParamsRequest {
                subspace_id: TEST_SUBSPACE.into(),
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryReactionsParamsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = SubspaceReactionsParams {
            subspace_id: TEST_SUBSPACE.into(),
            registered_reaction: Some(RegisteredReactionValueParams { enabled: true }),
            free_text: Some(FreeTextValueParams {
                enabled: true,
                max_length: 5,
                reg_ex: "".to_string(),
            }),
        };
        assert_eq!(expected, result.params.unwrap());
    }
}
