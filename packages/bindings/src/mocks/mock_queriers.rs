//! Contains some useful functions to perform unit testing of smart contracts.

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_slice, Binary, Coin, Empty, OwnedDeps, Querier, QuerierResult,
    QueryRequest, SystemError, SystemResult,
};
use mock::MockableQuerier;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Custom querier that can be used during unit testing to simulate what a contract receive when
/// perform a query toward Desmos’s modules.
pub struct MockDesmosQuerier {
    /// Default CosmWASM mock querier.
    pub mock_querier: MockQuerier,
    /// Registered custom queries using proto request for testing.
    pub registered_custom_queries: HashMap<String, Box<dyn Fn(&Binary) -> QuerierResult>>,
}

impl MockDesmosQuerier {
    /// Initialize a new [`MockDesmosQuerier`].
    /// * `balances` - Slice of balances passed to the `bank` module, where the first element
    /// is the user address and the second element is the user's balance.
    pub fn new(balances: &[(&str, &[Coin])]) -> Self {
        let mut querier = MockDesmosQuerier {
            mock_querier: MockQuerier::new(balances),
            registered_custom_queries: HashMap::new(),
        };
        register_default_mock_queries(&mut querier);
        querier
    }
    /// Handle the query request.
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match request {
            QueryRequest::Stargate { path, data } => {
                if let Some(response_fn) = self.registered_custom_queries.get(path) {
                    return response_fn(data);
                }
                SystemResult::Err(SystemError::UnsupportedRequest { kind: path.into() })
            }
            _ => self.mock_querier.handle_query(request),
        }
    }
}

impl MockableQuerier for MockDesmosQuerier {
    fn register_custom_query(
        &mut self,
        path: String,
        response_fn: Box<dyn Fn(&Binary) -> QuerierResult>,
    ) {
        self.registered_custom_queries.insert(path, response_fn);
    }
}

impl Querier for MockDesmosQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl Default for MockDesmosQuerier {
    fn default() -> Self {
        MockDesmosQuerier::new(&[])
    }
}

/// Creates an instance of [`OwnedDeps`](cosmwasm_std::OwnedDeps) with a custom [`MockDesmosQuerier`]
/// to allow the user to mock the query responses of one or more Desmos's modules.
///
pub fn mock_desmos_dependencies_with_custom_querier(
    querier: MockDesmosQuerier,
) -> OwnedDeps<MockStorage, MockApi, MockDesmosQuerier, Empty> {
    OwnedDeps::<_, _, _, Empty> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier,
        custom_query_type: PhantomData,
    }
}

/// Creates an instance of [`OwnedDeps`](cosmwasm_std::OwnedDeps) that is capable of
/// handling queries towards Desmos's modules.
pub fn mock_desmos_dependencies() -> OwnedDeps<MockStorage, MockApi, MockDesmosQuerier, Empty> {
    mock_desmos_dependencies_with_custom_querier(MockDesmosQuerier::default())
}


fn register_default_mock_queries(querier: &mut MockDesmosQuerier) {
    #[cfg(feature = "posts")]
    {
        use crate::posts::proto::{QuerySubspacePostsRequest, QuerySectionPostsRequest, QueryPostRequest, QueryPostAttachmentsRequest, QueryPollAnswersRequest};
        use crate::posts::mocks::MockPostsQueries;

        QuerySubspacePostsRequest::mock_response(querier, MockPostsQueries::get_mocked_subspace_posts_response());
        QuerySectionPostsRequest::mock_response(querier, MockPostsQueries::get_mocked_section_posts_response());
        QueryPostRequest::mock_response(querier, MockPostsQueries::get_mocked_post_response());
        QueryPostAttachmentsRequest::mock_response(querier, MockPostsQueries::get_mocked_post_attachments_response());
        QueryPollAnswersRequest::mock_response(querier, MockPostsQueries::get_mocked_poll_answers_response());
    }
}