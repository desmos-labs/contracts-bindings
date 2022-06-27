#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_SUBSPACE, USER1_ADDRESS, TEST_REACTIONS_POST_ID, TEST_REGISTERED_REACTION_ID,
        TEST_POST_REGISTERED_REACTION_ID, TEST_POST_FREE_TEXT_REACTION_ID, TEST_POST_DELETABLE_REACTION_ID
    };
    use cosmwasm_std::Addr;
    use desmos_bindings::reactions::{
        models_query::{
            QueryReactionsResponse, QueryReactionResponse,
            QueryRegisteredReactionsResponse, QueryRegisteredReactionResponse,
            QueryReactionsParamsResponse,
        },
        models::{
            Reaction, ReactionValue, RegisteredReaction, 
            SubspaceReactionsParams, RegisteredReactionValueParams, FreeTextValueParams,
        },
    };
    use desmos_bindings::reactions::query::ReactionsQuery;
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_post_reactions() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ReactionsQuery::Reactions {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryReactionsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = vec![
            Reaction {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                id: TEST_POST_REGISTERED_REACTION_ID,
                value: ReactionValue::Registered{ registered_reaction_id:  TEST_EDITABLE_REGISTERED_REACTION_ID },
                user: Addr::unchecked(contract_address),
            },
            Reaction {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                id: TEST_POST_FREE_TEXT_REACTION_ID,
                value: ReactionValue::Registered{ text:  "test_text".to_string() },
                user: Addr::unchecked(contract_address),
            },
            Reaction {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                id: TEST_POST_DELETABLE_REACTION_ID,
                value: ReactionValue::Registered{ text:  "deletable_text".to_string() },
                user: Addr::unchecked(contract_address),
            },
        ];
        assert_eq!(expected, result.reactions);
    }

    #[test]
    fn test_query_post_reaction() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ReactionsQuery::Reaction {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                reaction_id: TEST_POST_REGISTERED_REACTION_ID,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryReactionResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = Reaction {
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                id: TEST_POST_REGISTERED_REACTION_ID,
                value: ReactionValue::Registered{ registered_reaction_id:  TEST_EDITABLE_REGISTERED_REACTION_ID },
                user: Addr::unchecked(contract_address),
            };
        assert_eq!(expected, result.reaction);
    }

    #[test]
    fn test_query_registered_reactions() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ReactionsQuery::RegisteredReactions {
                subspace_id: TEST_SUBSPACE,
                pagination: None,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryRegisteredReactionsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = vec![
            RegisteredReaction{}
        ];
        assert_eq!(expected, result.reaction);
    }

    #[test]
    fn test_query_registered_reaction() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ReactionsQuery::RegisteredReaction {
                subspace_id: TEST_SUBSPACE,
                registered_reaction_id: 1,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryRegisteredReactionResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = RegisteredReaction {
            subspace_id: TEST_SUBSPACE,
            shorthand_code: "editable_code".to_string(),
            display_value: "editable_value".to_string(),
            user: Addr::unchecked(contract_address),
        }
        assert_eq!(expected, result.reaction);
    }

    #[test]
    fn test_query_reactions_params() {
        let desmos_cli = DesmosCli::default();

        let query_msg = DesmosChain {
            request: ReactionsQuery::ReactionsParams {
                subspace_id: TEST_SUBSPACE,
            }
            .into(),
        };

        let contract_address = desmos_cli.get_contract_by_code(1);

        let result: QueryReactionsParamsResponse = desmos_cli
            .wasm_query(&contract_address, &query_msg)
            .to_object();

        let expected = SubspaceReactionsParams{
            subspace_id: TEST_SUBSPACE,
            registered_reaction: RegisteredReactionValueParams { enabled: true },
            free_text: FreeTextValueParams {enable: true, reg_ex: "" },
        }
        assert_eq!(expected, result.reaction);
    }
}