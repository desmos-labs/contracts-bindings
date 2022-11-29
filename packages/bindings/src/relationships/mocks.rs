//! Contains some useful mocks of the Desmos x/relationships module's types made to be used in any test.

/// Struct that contains some utility methods to mock data of the Desmos
/// x/relationships module.
pub struct MockRelationshipsQueries {}

pub const MOCK_USER : &'static str = "user";
pub const MOCK_TARGET: &'static str = "target";

impl MockRelationshipsQueries {
    /// Get a mocked [`Relationship`].
    pub fn get_mock_relationship() -> Relationship {
        Relationship {
            creator: Addr::unchecked(MOCK_USER),
            counterparty: Addr::unchecked(MOCK_TARGET),
            subspace_id: Uint64::new(1),
        }
    }

    /// Get a mocked [`UserBlock`].
    pub fn get_mock_user_block() -> UserBlock {
        UserBlock {
            blocker: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            blocked: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
            reason: "test".to_string(),
            subspace_id: Uint64::new(1),
        }
    }
}