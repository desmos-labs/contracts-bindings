//! Contains useful functions to perform unit testing of smart contracts.

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_slice, Binary, Coin, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError,
    SystemResult,
};

use mock::MockableQuerier;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Custom querier that can be used during unit testing to simulate what a contract receive when
/// perform a query toward Desmos’s modules.
pub struct MockDesmosQuerier<C: DeserializeOwned = Empty> {
    /// Default CosmWASM mock querier.
    pub mock_querier: MockQuerier<C>,
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
        use crate::posts::mocks::MockPostsQueries;
        use crate::posts::types::{
            QueryPollAnswersRequest, QueryPostAttachmentsRequest, QueryPostRequest,
            QuerySectionPostsRequest, QuerySubspacePostsRequest,
        };

        QuerySubspacePostsRequest::mock_response(
            querier,
            MockPostsQueries::get_mocked_subspace_posts_response(),
        );

        QuerySectionPostsRequest::mock_response(
            querier,
            MockPostsQueries::get_mocked_section_posts_response(),
        );
        QueryPostRequest::mock_response(querier, MockPostsQueries::get_mocked_post_response());

        QueryPostAttachmentsRequest::mock_response(
            querier,
            MockPostsQueries::get_mocked_post_attachments_response(),
        );

        QueryPollAnswersRequest::mock_response(
            querier,
            MockPostsQueries::get_mocked_poll_answers_response(),
        );
    }
    #[cfg(feature = "profiles")]
    {
        use crate::profiles::mocks::MockProfilesQueries;
        use crate::profiles::types::{
            QueryApplicationLinkByClientIdRequest, QueryApplicationLinkOwnersRequest,
            QueryApplicationLinksRequest, QueryChainLinkOwnersRequest, QueryChainLinksRequest,
            QueryDefaultExternalAddressesRequest, QueryIncomingDTagTransferRequestsRequest,
            QueryProfileRequest,
        };

        QueryProfileRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_profile_response(),
        );

        QueryIncomingDTagTransferRequestsRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_incoming_dtag_transfer_requests_response(),
        );

        QueryChainLinksRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_chain_links_response(),
        );

        QueryChainLinkOwnersRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_chain_link_owners_response(),
        );

        QueryDefaultExternalAddressesRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_default_external_addresses_response(),
        );

        QueryApplicationLinksRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_application_links_response(),
        );

        QueryApplicationLinkByClientIdRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_application_link_by_client_id_response(),
        );

        QueryApplicationLinkOwnersRequest::mock_response(
            querier,
            MockProfilesQueries::get_mocked_application_link_owners_response(),
        );
    }
    #[cfg(feature = "reactions")]
    {
        use crate::reactions::mocks::MockReactionsQueries;
        use crate::reactions::types::{
            QueryReactionRequest, QueryReactionsParamsRequest, QueryReactionsRequest,
            QueryRegisteredReactionRequest, QueryRegisteredReactionsRequest,
        };

        QueryReactionRequest::mock_response(
            querier,
            MockReactionsQueries::get_mocked_reaction_response(),
        );

        QueryReactionsRequest::mock_response(
            querier,
            MockReactionsQueries::get_mocked_reactions_response(),
        );

        QueryRegisteredReactionRequest::mock_response(
            querier,
            MockReactionsQueries::get_mocked_registered_reaction_response(),
        );

        QueryRegisteredReactionsRequest::mock_response(
            querier,
            MockReactionsQueries::get_mocked_registered_reactions_response(),
        );

        QueryReactionsParamsRequest::mock_response(
            querier,
            MockReactionsQueries::get_mocked_reactions_params_response(),
        );
    }
    #[cfg(feature = "relationships")]
    {
        use crate::relationships::mocks::MockRelationshipsQueries;
        use crate::relationships::types::{QueryBlocksRequest, QueryRelationshipsRequest};

        QueryRelationshipsRequest::mock_response(
            querier,
            MockRelationshipsQueries::get_mocked_relationships_response(),
        );

        QueryBlocksRequest::mock_response(
            querier,
            MockRelationshipsQueries::get_mocked_blocks_response(),
        );
    }
    #[cfg(feature = "reports")]
    {
        use crate::reports::mocks::MockReportsQueries;
        use crate::reports::types::{
            QueryReasonRequest, QueryReasonsRequest, QueryReportRequest, QueryReportsRequest,
        };

        QueryReportsRequest::mock_response(
            querier,
            MockReportsQueries::get_mocked_reports_response(),
        );

        QueryReportRequest::mock_response(
            querier,
            MockReportsQueries::get_mocked_report_response(),
        );

        QueryReasonsRequest::mock_response(
            querier,
            MockReportsQueries::get_mocked_reasons_response(),
        );

        QueryReasonRequest::mock_response(
            querier,
            MockReportsQueries::get_mocked_reason_response(),
        );
    }
    #[cfg(feature = "subspaces")]
    {
        use crate::subspaces::mocks::MockSubspacesQueries;
        use crate::subspaces::types::{
            QuerySectionRequest, QuerySectionsRequest, QuerySubspaceRequest, QuerySubspacesRequest,
            QueryUserGroupMembersRequest, QueryUserGroupRequest, QueryUserGroupsRequest,
            QueryUserPermissionsRequest,
        };

        QuerySubspacesRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_subspaces_response(),
        );

        QuerySubspaceRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_subspace_response(),
        );

        QuerySectionsRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_sections_response(),
        );

        QuerySectionRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_section_response(),
        );

        QueryUserGroupsRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_user_groups_response(),
        );

        QueryUserGroupRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_user_group_response(),
        );

        QueryUserGroupMembersRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_user_group_members_response(),
        );

        QueryUserPermissionsRequest::mock_response(
            querier,
            MockSubspacesQueries::get_mocked_user_permissions_response(),
        );
    }
}
