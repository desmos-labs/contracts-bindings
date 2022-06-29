//! Contains some useful mocks of the Desmos x/reactions module's types made to be used in any test.

use crate::reactions::{
    models::{
        FreeTextValueParams, Reaction, ReactionValue, RegisteredReaction,
        RegisteredReactionValueParams, SubspaceReactionsParams,
    },
    models_query::{
        QueryReactionResponse, QueryReactionsParamsResponse, QueryReactionsResponse,
        QueryRegisteredReactionResponse, QueryRegisteredReactionsResponse,
    },
    query::ReactionsQuery,
};
use cosmwasm_std::{to_binary, Addr, Binary, ContractResult, Uint64};

/// Struct that contains some utility methods to mock data of the Desmos
/// x/reactions module.
pub struct MockReactionsQueries {}

impl MockReactionsQueries {
    /// Get a mocked [`Reaction`].
    pub fn get_mock_reaction() -> Reaction {
        Reaction {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            id: 1,
            value: ReactionValue::FreeText {
                text: "test".to_string(),
            }
            .into(),
            author: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
        }
    }

    /// Get a mocked [`RegisteredReaction`].
    pub fn get_mock_registered_reaction() -> RegisteredReaction {
        RegisteredReaction {
            subspace_id: Uint64::new(1),
            id: 1,
            shorthand_code: "code".to_string(),
            display_value: "value".to_string(),
        }
    }

    /// Get a mocked [`ReactionsParams`]
    pub fn get_mock_reactions_parameters() -> SubspaceReactionsParams {
        SubspaceReactionsParams {
            subspace_id: Uint64::new(1),
            registered_reaction: RegisteredReactionValueParams { enabled: true },
            free_text: FreeTextValueParams {
                enabled: true,
                max_length: 100,
                reg_ex: "".to_string(),
            },
        }
    }
}

/// Functions that mocks the reactions query responses.
pub fn mock_reactions_query_response(query: &ReactionsQuery) -> ContractResult<Binary> {
    let response = match query {
        ReactionsQuery::Reactions { .. } => {
            let reaction = MockReactionsQueries::get_mock_reaction();
            to_binary(&QueryReactionsResponse {
                reactions: vec![reaction],
                pagination: Default::default(),
            })
        }
        ReactionsQuery::Reaction { .. } => {
            let reaction = MockReactionsQueries::get_mock_reaction();
            to_binary(&QueryReactionResponse { reaction: reaction })
        }
        ReactionsQuery::RegisteredReactions { .. } => {
            let registered_reaction = MockReactionsQueries::get_mock_registered_reaction();
            to_binary(&QueryRegisteredReactionsResponse {
                registered_reactions: vec![registered_reaction],
                pagination: Default::default(),
            })
        }
        ReactionsQuery::RegisteredReaction { .. } => {
            let registered_reaction = MockReactionsQueries::get_mock_registered_reaction();
            to_binary(&QueryRegisteredReactionResponse {
                registered_reaction: registered_reaction,
            })
        }
        ReactionsQuery::ReactionsParams { .. } => {
            let params = MockReactionsQueries::get_mock_reactions_parameters();
            to_binary(&QueryReactionsParamsResponse { params: params })
        }
    };
    response.into()
}

#[cfg(test)]
mod tests {
    use crate::reactions::{
        mocks::{mock_reactions_query_response, MockReactionsQueries},
        models_query::{
            QueryReactionResponse, QueryReactionsParamsResponse, QueryReactionsResponse,
            QueryRegisteredReactionResponse, QueryRegisteredReactionsResponse,
        },
        query::ReactionsQuery,
    };
    use cosmwasm_std::{to_binary, Uint64};

    #[test]
    fn test_query_reactions() {
        let query = ReactionsQuery::Reactions {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            user: None,
            pagination: Default::default(),
        };
        let response = mock_reactions_query_response(&query);
        let expected = to_binary(&QueryReactionsResponse {
            reactions: vec![MockReactionsQueries::get_mock_reaction()],
            pagination: Default::default(),
        });
        println!("{:?}", response.clone().into_result().ok());
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_reaction() {
        let query = ReactionsQuery::Reaction {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            reaction_id: 1,
        };
        let response = mock_reactions_query_response(&query);
        let expected = to_binary(&QueryReactionResponse {
            reaction: MockReactionsQueries::get_mock_reaction(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_registered_reactions() {
        let query = ReactionsQuery::RegisteredReactions {
            subspace_id: Uint64::new(1),
            pagination: Default::default(),
        };
        let response = mock_reactions_query_response(&query);
        let expected = to_binary(&QueryRegisteredReactionsResponse {
            registered_reactions: vec![MockReactionsQueries::get_mock_registered_reaction()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_registered_reaction() {
        let query = ReactionsQuery::RegisteredReaction {
            subspace_id: Uint64::new(1),
            reaction_id: 1,
        };
        let response = mock_reactions_query_response(&query);
        let expected = to_binary(&QueryRegisteredReactionResponse {
            registered_reaction: MockReactionsQueries::get_mock_registered_reaction(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_reactions_params() {
        let query = ReactionsQuery::ReactionsParams {
            subspace_id: Uint64::new(1),
        };
        let response = mock_reactions_query_response(&query);
        let expected = to_binary(&QueryReactionsParamsResponse {
            params: MockReactionsQueries::get_mock_reactions_parameters(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }
}
