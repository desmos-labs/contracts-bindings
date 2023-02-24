//! Contains useful mocks of the Desmos x/relationships module's types made to be used in any test.

use crate::relationships::types::{
    QueryBlocksResponse, QueryRelationshipsResponse, Relationship, UserBlock,
};

/// Struct that contains some utility methods to mock data of the Desmos
/// x/relationships module.
pub struct MockRelationshipsQueries {}

/// Represents the mock user of the relationship for unit test.
pub const MOCK_USER: &'static str = "user";

/// Represents the mock target of the relationship for unit test.
pub const MOCK_TARGET: &'static str = "target";

impl MockRelationshipsQueries {
    /// Gets a mocked [`Relationship`].
    pub fn get_mocked_relationship(subspace_id: u64) -> Relationship {
        Relationship {
            creator: MOCK_USER.into(),
            counterparty: MOCK_TARGET.into(),
            subspace_id,
        }
    }

    /// Gets a mocked [`UserBlock`].
    pub fn get_mocked_user_block(subspace_id: u64) -> UserBlock {
        UserBlock {
            blocker: MOCK_USER.into(),
            blocked: MOCK_TARGET.into(),
            reason: "test".to_string(),
            subspace_id,
        }
    }
    /// Gets a mocked [`QueryRelationshipsResponse`].
    pub fn get_mocked_relationships_response() -> QueryRelationshipsResponse {
        QueryRelationshipsResponse {
            relationships: vec![Self::get_mocked_relationship(1)],
            pagination: None,
        }
    }
    /// Gets a mocked [`QueryBlocksResponse`].
    pub fn get_mocked_blocks_response() -> QueryBlocksResponse {
        QueryBlocksResponse {
            blocks: vec![Self::get_mocked_user_block(1)],
            pagination: None,
        }
    }
}
