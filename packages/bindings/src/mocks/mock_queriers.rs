//! Contains some useful functions to perform unit testing of smart contracts.

#[cfg(feature = "posts")]
use crate::posts::mocks::mock_posts_query_response;
#[cfg(feature = "profiles")]
use crate::profiles::mocks::mock_profiles_query_response;
use crate::query::DesmosQuery;
#[cfg(feature = "reactions")]
use crate::reactions::mocks::mock_reactions_query_response;
#[cfg(feature = "relationships")]
use crate::relationships::mocks::mock_relationships_query_response;
#[cfg(feature = "reports")]
use crate::reports::mocks::mock_reports_query_response;
#[cfg(feature = "subspaces")]
use crate::subspaces::mocks::mock_subspaces_query_response;
use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Coin, CustomQuery, OwnedDeps, SystemError, SystemResult,
};
use std::marker::PhantomData;

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier to use desmos querier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQuery>, DesmosQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier = MockQuerier::<DesmosQuery>::new(&[(contract_addr, contract_balance)])
        .with_custom_handler(|query| match query {
            #[cfg(feature = "profiles")]
            DesmosQuery::Profiles(query) => SystemResult::Ok(mock_profiles_query_response(query)),
            #[cfg(feature = "subspaces")]
            DesmosQuery::Subspaces(query) => SystemResult::Ok(mock_subspaces_query_response(query)),
            #[cfg(feature = "relationships")]
            DesmosQuery::Relationships(query) => {
                SystemResult::Ok(mock_relationships_query_response(query))
            }
            #[cfg(feature = "posts")]
            DesmosQuery::Posts(query) => SystemResult::Ok(mock_posts_query_response(query)),
            #[cfg(feature = "reactions")]
            DesmosQuery::Reactions(query) => SystemResult::Ok(mock_reactions_query_response(query)),
            #[cfg(feature = "reports")]
            DesmosQuery::Reports(query) => SystemResult::Ok(mock_reports_query_response(query)),
            // Hide this warning since when we compile the package without any module feature
            // this pattern is reached.
            #[allow(unreachable_patterns)]
            _ => SystemResult::Err(SystemError::Unknown {}),
        });
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
