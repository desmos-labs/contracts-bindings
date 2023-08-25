//! Contains useful mocks of the Desmos x/tokenfactory module's types made to be used in any test.

use crate::tokenfactory::types::QuerySubspaceDenomsResponse;

/// Struct that contains some utility methods to mock data of the Desmos
/// x/tokenfactory module.
pub struct MockTokenfactoryQueries {}

impl MockTokenfactoryQueries {
    /// Gets a mocked instance of [`QuerySubspaceDenomsResponse`].
    pub fn get_mocked_subspace_denoms_response() -> QuerySubspaceDenomsResponse {
        QuerySubspaceDenomsResponse {
            denoms: vec!["denom-1".into(), "denom-2".into()],
        }
    }
}
