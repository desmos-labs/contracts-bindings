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
        models::{Reaction, ReactionValue},
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
            Reaction{
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                id: TEST_POST_REGISTERED_REACTION_ID,
                value: ReactionValue::Registered{ registered_reaction_id:  TEST_EDITABLE_REGISTERED_REACTION_ID },
                user: Addr::unchecked(contract_address),
            },
            Reaction{
                subspace_id: TEST_SUBSPACE,
                post_id: TEST_REACTIONS_POST_ID,
                id: TEST_POST_FREE_TEXT_REACTION_ID,
                value: ReactionValue::Registered{ text:  "test_text".to_string() },
                user: Addr::unchecked(contract_address),
            },
        ];

        assert_eq!(expected, result.reactions, false);
    }
}