//! Contains some useful mocks of the Desmos x/reactions module's types made to be used in any test.

use crate::stargate::reactions::types::{
    FreeTextValue, FreeTextValueParams, QueryReactionResponse, QueryReactionsParamsResponse,
    QueryReactionsResponse, QueryRegisteredReactionResponse, QueryRegisteredReactionsResponse,
    Reaction, RegisteredReaction, RegisteredReactionValueParams, SubspaceReactionsParams,
};

pub const MOCK_REACTION_AUTHOR: &str = "author";

// Struct that contains some utility methods to mock data of the Desmos
/// x/reactions module.
pub struct MockReactionsQueries {}

impl MockReactionsQueries {
    /// Gets a mocked [`Reaction`].
    pub fn get_mocked_reaction(subspace_id: u64, post_id: u64, id: u32) -> Reaction {
        Reaction {
            subspace_id,
            post_id,
            id,
            value: Some(
                FreeTextValue {
                    text: "test".into(),
                }
                .into(),
            ),
            author: MOCK_REACTION_AUTHOR.into(),
        }
    }
    /// Gets a mocked [`RegisteredReaction`].
    pub fn get_mocked_registered_reaction(subspace_id: u64, id: u32) -> RegisteredReaction {
        RegisteredReaction {
            subspace_id,
            id,
            shorthand_code: "code".into(),
            display_value: "value".into(),
        }
    }
    /// Gets a mocked [`SubspaceReactionsParams`].
    pub fn get_mocked_subspace_reactions_params(subspace_id: u64) -> SubspaceReactionsParams {
        SubspaceReactionsParams {
            subspace_id,
            registered_reaction: Some(RegisteredReactionValueParams { enabled: true }),
            free_text: Some(FreeTextValueParams {
                enabled: true,
                max_length: 100,
                reg_ex: "".into(),
            }),
        }
    }
    /// Function that mocks a [`QueryReactionsResponse`].
    pub fn get_mocked_reactions_response() -> QueryReactionsResponse {
        QueryReactionsResponse {
            reactions: vec![Self::get_mocked_reaction(1, 1, 1)],
            pagination: None,
        }
    }
    /// Function that mocks a [`QueryReactionResponse`].
    pub fn get_mocked_reaction_response() -> QueryReactionResponse {
        QueryReactionResponse {
            reaction: Some(Self::get_mocked_reaction(1, 1, 1)),
        }
    }
    /// Function that mocks a [`QueryRegisteredReactionsResponse`].
    pub fn get_mocked_registered_reactions_response() -> QueryRegisteredReactionsResponse {
        QueryRegisteredReactionsResponse {
            registered_reactions: vec![Self::get_mocked_registered_reaction(1, 1)],
            pagination: None,
        }
    }
    /// Function that mocks a [`QueryRegisteredReactionResponse`].
    pub fn get_mocked_registered_reaction_response() -> QueryRegisteredReactionResponse {
        QueryRegisteredReactionResponse {
            registered_reaction: Some(Self::get_mocked_registered_reaction(1, 1)),
        }
    }
    /// Function that mocks a [`QueryReactionsParamsResponse`].
    pub fn get_mocked_reactions_params_response() -> QueryReactionsParamsResponse {
        QueryReactionsParamsResponse {
            params: Some(Self::get_mocked_subspace_reactions_params(1)),
        }
    }
}
