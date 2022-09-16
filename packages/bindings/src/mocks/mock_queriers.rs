//! Contains some useful functions to perform unit testing of smart contracts.

#[cfg(feature = "posts")]
use crate::posts::{mocks::mock_posts_query_response, query::PostsQuery};
#[cfg(feature = "profiles")]
use crate::profiles::{mocks::mock_profiles_query_response, query::ProfilesQuery};
use crate::query::DesmosQuery;
#[cfg(feature = "reactions")]
use crate::reactions::{mocks::mock_reactions_query_response, query::ReactionsQuery};
#[cfg(feature = "relationships")]
use crate::relationships::{mocks::mock_relationships_query_response, query::RelationshipsQuery};
#[cfg(feature = "reports")]
use crate::reports::{mocks::mock_reports_query_response, query::ReportsQuery};
#[cfg(feature = "subspaces")]
use crate::subspaces::{mocks::mock_subspaces_query_response, query::SubspacesQuery};
use cosmwasm_std::testing::MockQuerierCustomHandlerResult;
use cosmwasm_std::{
    from_slice,
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Binary, Coin, ContractResult, OwnedDeps, Querier, QuerierResult, QueryRequest, StdResult,
    SystemError, SystemResult,
};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Custom querier that can be used during unit testing to simulate what a contract receive when
/// perform a query.
pub struct MockDesmosQuerier {
    /// Default CosmWASM mock querier.
    pub mock_querier: MockQuerier<DesmosQuery>,
    #[cfg(feature = "profiles")]
    profiles_handler: Box<dyn for<'a> Fn(&'a ProfilesQuery) -> MockQuerierCustomHandlerResult>,
    #[cfg(feature = "subspaces")]
    subspaces_handler: Box<dyn for<'a> Fn(&'a SubspacesQuery) -> MockQuerierCustomHandlerResult>,
    #[cfg(feature = "posts")]
    posts_handler: Box<dyn for<'a> Fn(&'a PostsQuery) -> MockQuerierCustomHandlerResult>,
    #[cfg(feature = "relationships")]
    relationships_handler:
        Box<dyn for<'a> Fn(&'a RelationshipsQuery) -> MockQuerierCustomHandlerResult>,
    #[cfg(feature = "reports")]
    reports_handler: Box<dyn for<'a> Fn(&'a ReportsQuery) -> MockQuerierCustomHandlerResult>,
    #[cfg(feature = "reactions")]
    reactions_handler: Box<dyn for<'a> Fn(&'a ReactionsQuery) -> MockQuerierCustomHandlerResult>,
}

impl MockDesmosQuerier {
    /// Initialize a new [`MockDesmosQuerier`].
    /// * `balances` - Slice of balances passed to the `bank` module, where the first element
    /// is the user address and the second element is the user's balance.
    pub fn new(balances: &[(&str, &[Coin])]) -> Self {
        MockDesmosQuerier {
            mock_querier: MockQuerier::new(balances),
            profiles_handler: Box::new(|q| SystemResult::Ok(mock_profiles_query_response(q))),
            subspaces_handler: Box::new(|q| SystemResult::Ok(mock_subspaces_query_response(q))),
            posts_handler: Box::new(|q| SystemResult::Ok(mock_posts_query_response(q))),
            relationships_handler: Box::new(|q| {
                SystemResult::Ok(mock_relationships_query_response(q))
            }),
            reports_handler: Box::new(|q| SystemResult::Ok(mock_reports_query_response(q))),
            reactions_handler: Box::new(|q| SystemResult::Ok(mock_reactions_query_response(q))),
        }
    }

    /// Handle the query request.
    pub fn handle_query(&self, request: &QueryRequest<DesmosQuery>) -> QuerierResult {
        match request {
            QueryRequest::Custom(desmos_query) => {
                match desmos_query {
                    #[cfg(feature = "profiles")]
                    DesmosQuery::Profiles(query) => (*self.profiles_handler)(query),
                    #[cfg(feature = "subspaces")]
                    DesmosQuery::Subspaces(query) => (*self.subspaces_handler)(query),
                    #[cfg(feature = "posts")]
                    DesmosQuery::Posts(query) => (*self.posts_handler)(query),
                    #[cfg(feature = "relationships")]
                    DesmosQuery::Relationships(query) => (*self.relationships_handler)(query),
                    #[cfg(feature = "reports")]
                    DesmosQuery::Reports(query) => (*self.reports_handler)(query),
                    #[cfg(feature = "reactions")]
                    DesmosQuery::Reactions(query) => (*self.reactions_handler)(query),
                    // Hide this warning since when we compile the package without any module feature
                    // this pattern is reached.
                    #[allow(unreachable_patterns)]
                    _ => SystemResult::Err(SystemError::Unknown {}),
                }
            }
            _ => self.mock_querier.handle_query(request),
        }
    }

    /// Utility function to wrap the handler that returns a ContractResult<Binary>
    /// to make it return a SystemResult<ContractResult<Binary>>
    fn wrap_handler<'f, CH, Q>(
        handler: CH,
    ) -> Box<dyn for<'a> Fn(&Q) -> SystemResult<ContractResult<Binary>>>
    where
        CH: Fn(&Q) -> ContractResult<Binary> + 'static,
        Q: DeserializeOwned,
    {
        Box::new(move |query| {
            let result = handler(query);
            if result.is_ok() {
                SystemResult::Ok(result)
            } else {
                SystemResult::Err(SystemError::UnsupportedRequest {
                    kind: result.unwrap_err(),
                })
            }
        })
    }

    /// Function to provide an user defined handler to mock responses to requests made to
    /// the desmos `profiles` module.
    /// * `handler` - Function that will be called when the contract under test performs a query
    /// towards the `profiles` module.
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{ContractResult, SystemError, SystemResult, to_binary};
    /// use desmos_bindings::mocks::mock_queriers::MockDesmosQuerier;
    /// use desmos_bindings::profiles::mocks::MockProfilesQueries;
    /// use desmos_bindings::profiles::models_profile::Profile;
    /// use desmos_bindings::profiles::models_query::QueryProfileResponse;
    /// use desmos_bindings::profiles::query::ProfilesQuery;
    ///
    /// let querier =
    ///     MockDesmosQuerier::default().with_custom_profiles_handler(|query| match query {
    ///         ProfilesQuery::Profile { user } => to_binary(&QueryProfileResponse {
    ///             profile: MockProfilesQueries::get_mock_profile(),
    ///         })
    ///         .into(),
    ///         _ => ContractResult::Err("not supported".to_string()),
    ///     });
    /// ```
    #[cfg(feature = "profiles")]
    pub fn with_custom_profiles_handler<CH>(mut self, handler: CH) -> Self
    where
        CH: Fn(&ProfilesQuery) -> ContractResult<Binary> + 'static,
    {
        self.profiles_handler = MockDesmosQuerier::wrap_handler(handler);
        self
    }

    /// Function to provide an user defined handler to mock responses to requests made to
    /// the desmos `subspaces` module.
    /// * `handler` - Function that will be called when the contract under test performs a query
    /// towards the `subspaces` module.
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{ContractResult, to_binary};
    /// use desmos_bindings::mocks::mock_queriers::MockDesmosQuerier;
    /// use desmos_bindings::subspaces::mocks::MockSubspacesQueries;
    /// use desmos_bindings::subspaces::models_query::QuerySubspaceResponse;
    /// use desmos_bindings::subspaces::query::SubspacesQuery;
    ///
    /// let querier =
    ///     MockDesmosQuerier::default().with_custom_subspaces_handler(|query| match query {
    ///         SubspacesQuery::Subspace { subspace_id } => to_binary(&QuerySubspaceResponse {
    ///             subspace: MockSubspacesQueries::get_mock_subspace(),
    ///         })
    ///         .into(),
    ///         _ => ContractResult::Err("not supported".to_string()),
    ///     });
    /// ```
    #[cfg(feature = "subspaces")]
    pub fn with_custom_subspaces_handler<CH>(mut self, handler: CH) -> Self
    where
        CH: Fn(&SubspacesQuery) -> ContractResult<Binary> + 'static,
    {
        self.subspaces_handler = MockDesmosQuerier::wrap_handler(handler);
        self
    }

    /// Function to provide an user defined handler to mock responses to requests made to
    /// the desmos `post` module.
    /// * `handler` - Function that will be called when the contract under test performs a query
    /// towards the `post` module.
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{ContractResult, to_binary};
    /// use desmos_bindings::mocks::mock_queriers::MockDesmosQuerier;
    /// use desmos_bindings::posts::mocks::MockPostsQueries;
    /// use desmos_bindings::posts::models_query::QueryPostResponse;
    /// use desmos_bindings::posts::query::PostsQuery;
    ///
    /// let querier =
    ///     MockDesmosQuerier::default().with_custom_posts_handler(|query| match query {
    ///         PostsQuery::Post { subspace_id, post_id } => to_binary(&QueryPostResponse {
    ///             post: MockPostsQueries::get_mocked_post(subspace_id.clone(), post_id.clone()),
    ///         })
    ///         .into(),
    ///         _ => ContractResult::Err("not supported".to_string()),
    ///     });
    /// ```
    #[cfg(feature = "posts")]
    pub fn with_custom_posts_handler<CH>(mut self, handler: CH) -> Self
    where
        CH: Fn(&PostsQuery) -> ContractResult<Binary> + 'static,
    {
        self.posts_handler = MockDesmosQuerier::wrap_handler(handler);
        self
    }

    /// Function to provide an user defined handler to mock responses to requests made to
    /// the desmos `relationships` module.
    /// * `handler` - Function that will be called when the contract under test performs a query
    /// towards the `relationships` module.
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{ContractResult, to_binary};
    /// use desmos_bindings::mocks::mock_queriers::MockDesmosQuerier;
    /// use desmos_bindings::relationships::mocks::MockRelationshipsQueries;
    /// use desmos_bindings::relationships::models_query::QueryRelationshipsResponse;
    /// use desmos_bindings::relationships::query::RelationshipsQuery;
    ///
    /// let querier =
    ///     MockDesmosQuerier::default().with_custom_relationships_handler(|query| match query {
    ///         RelationshipsQuery::Relationships { .. } => to_binary(&QueryRelationshipsResponse {
    ///             relationships: vec![MockRelationshipsQueries::get_mock_relationship()],
    ///             pagination: None,
    ///         })
    ///         .into(),
    ///         _ => ContractResult::Err("not supported".to_string()),
    ///     });
    /// ```
    #[cfg(feature = "relationships")]
    pub fn with_custom_relationships_handler<CH>(mut self, handler: CH) -> Self
    where
        CH: Fn(&RelationshipsQuery) -> ContractResult<Binary> + 'static,
    {
        self.relationships_handler = MockDesmosQuerier::wrap_handler(handler);
        self
    }

    /// Function to provide an user defined handler to mock responses to requests made to
    /// the desmos `reports` module.
    /// * `handler` - Function that will be called when the contract under test performs a query
    /// towards the `reports` module.
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{ContractResult, to_binary};
    /// use desmos_bindings::mocks::mock_queriers::MockDesmosQuerier;
    /// use desmos_bindings::reports::mocks::MockReportsQueries;
    /// use desmos_bindings::reports::models_query::QueryReportResponse;
    /// use desmos_bindings::reports::query::ReportsQuery;
    ///
    /// let querier =
    ///     MockDesmosQuerier::default().with_custom_reports_handler(|query| match query {
    ///         ReportsQuery::Report { subspace_id, report_id } => to_binary(&QueryReportResponse {
    ///             report: MockReportsQueries::get_mocked_report(subspace_id),
    ///         })
    ///         .into(),
    ///         _ => ContractResult::Err("not supported".to_string()),
    ///     });
    /// ```
    #[cfg(feature = "reports")]
    pub fn with_custom_reports_handler<CH>(mut self, handler: CH) -> Self
    where
        CH: Fn(&ReportsQuery) -> ContractResult<Binary> + 'static,
    {
        self.reports_handler = MockDesmosQuerier::wrap_handler(handler);
        self
    }

    /// Function to provide an user defined handler to mock responses to requests made to
    /// the desmos `reactions` module.
    /// * `handler` - Function that will be called when the contract under test performs a query
    /// towards the `reactions` module.
    ///
    /// # Example
    /// ```
    /// use cosmwasm_std::{ContractResult, to_binary};
    /// use desmos_bindings::mocks::mock_queriers::MockDesmosQuerier;
    /// use desmos_bindings::reactions::mocks::MockReactionsQueries;
    /// use desmos_bindings::reactions::models_query::QueryReactionResponse;
    /// use desmos_bindings::reactions::query::ReactionsQuery;
    ///
    /// let querier =
    ///     MockDesmosQuerier::default().with_custom_reactions_handler(|query| match query {
    ///         ReactionsQuery::Reaction { subspace_id, post_id, reaction_id } => to_binary(&QueryReactionResponse {
    ///             reaction: MockReactionsQueries::get_mock_reaction(),
    ///         })
    ///         .into(),
    ///         _ => ContractResult::Err("not supported".to_string()),
    ///     });
    /// ```
    #[cfg(feature = "reactions")]
    pub fn with_custom_reactions_handler<CH>(mut self, handler: CH) -> Self
    where
        CH: Fn(&ReactionsQuery) -> ContractResult<Binary> + 'static,
    {
        self.reactions_handler = MockDesmosQuerier::wrap_handler(handler);
        self
    }
}

impl Querier for MockDesmosQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<DesmosQuery> = match from_slice(bin_request) {
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

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier to use desmos querier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockDesmosQuerier, DesmosQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier = MockDesmosQuerier::new(&[(contract_addr, contract_balance)]);
    OwnedDeps::<_, _, _, DesmosQuery> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::mock_queriers::mock_dependencies_with_custom_querier,
        profiles::{
            mocks::MockProfilesQueries, models_query::QueryProfileResponse,
            querier::ProfilesQuerier,
        },
        reactions::{
            mocks::MockReactionsQueries, models_query::QueryReactionsResponse,
            querier::ReactionsQuerier,
        },
        relationships::{
            mocks::MockRelationshipsQueries, models_query::QueryRelationshipsResponse,
            querier::RelationshipsQuerier,
        },
        reports::{
            mocks::MockReportsQueries, models_query::QueryReportResponse, querier::ReportsQuerier,
        },
        subspaces::{
            mocks::MockSubspacesQueries, models_query::QuerySubspaceResponse,
            querier::SubspacesQuerier,
        },
    };
    use cosmwasm_std::{Addr, Uint64};
    use std::ops::Deref;

    #[test]
    fn test_profiles_querier_mock() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ProfilesQuerier::new(deps.querier.deref());
        let response = querier.query_profile(Addr::unchecked("")).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_subspaces_querier() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = SubspacesQuerier::new(deps.querier.deref());
        let response = querier.query_subspace(1).unwrap();
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        assert_eq!(expected, response);
    }

    #[test]
    fn test_relationships_querier() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = RelationshipsQuerier::new(deps.querier.deref());
        let response = querier
            .query_relationships(
                1,
                Some(Addr::unchecked("")),
                Some(Addr::unchecked("")),
                None,
            )
            .unwrap();
        let expected = QueryRelationshipsResponse {
            relationships: vec![MockRelationshipsQueries::get_mock_relationship()],
            pagination: Default::default(),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_reactions_querier() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ReactionsQuerier::new(deps.querier.deref());
        let response = querier.query_reactions(1, 1, None, None).unwrap();
        let expected = QueryReactionsResponse {
            reactions: vec![MockReactionsQueries::get_mock_reaction()],
            pagination: Default::default(),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_reports_querier() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let querier = ReportsQuerier::new(deps.querier.deref());
        let response = querier.query_report(1, 1).unwrap();
        let expected = QueryReportResponse {
            report: MockReportsQueries::get_mocked_report(&Uint64::new(1)),
        };
        assert_eq!(expected, response)
    }
}
