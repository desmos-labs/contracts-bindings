//! Contains integration test utils for desmos custom modules.

#![cfg(not(tarpaulin_include))]
use crate::mocks::mock_keeper::DesmosKeeper;
use crate::msg::DesmosMsg;
use crate::query::DesmosQuery;
use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{Addr, Api, Empty, GovMsg, IbcMsg, IbcQuery, Storage};
use cw_multi_test::{
    App, BankKeeper, BasicAppBuilder, DistributionKeeper, FailingModule, Module, Router,
    StakeKeeper, WasmKeeper,
};

/// Defines the module trait for desmos app.
pub trait DesmosModule: Module<ExecT = DesmosMsg, QueryT = DesmosQuery, SudoT = Empty> {}

impl DesmosModule for DesmosKeeper {}
impl DesmosModule for FailingModule<DesmosMsg, DesmosQuery, Empty> {}

/// DesmosApp wraps the desmos custom module into a mock app for integration tests.
/// It always returns successful response with proper events.
pub type DesmosApp<M = DesmosKeeper> =
    App<BankKeeper, MockApi, MockStorage, M, WasmKeeper<DesmosMsg, DesmosQuery>>;

/// Creates new default `DesmosApp` with customized initialization function.
pub fn custom_desmos_app<M, F>(custom: M, init_fn: F) -> DesmosApp<M>
where
    F: FnOnce(
        &mut Router<
            BankKeeper,
            M,
            WasmKeeper<DesmosMsg, DesmosQuery>,
            StakeKeeper,
            DistributionKeeper,
        >,
        &dyn Api,
        &mut dyn Storage,
    ),
    M: DesmosModule,
{
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom()
        .with_custom(custom)
        .build(init_fn)
}

/// Returns a mock failing app.
pub fn mock_failing_desmos_app() -> DesmosApp<FailingModule<DesmosMsg, DesmosQuery, Empty>> {
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom().build(|_, _, _| {})
}

/// Returns a mock default desmos app.
pub fn mock_desmos_app() -> DesmosApp {
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom()
        .with_custom(DesmosKeeper::new())
        .build(|_, _, _| {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        posts::{
            mocks::MockPostsQueries, models_query::QueryPostResponse, msg::PostsMsg,
            querier::PostsQuerier,
        },
        profiles::{
            mocks::MockProfilesQueries, models_query::QueryProfileResponse, msg::ProfilesMsg,
            querier::ProfilesQuerier,
        },
        reactions::{
            mocks::MockReactionsQueries, models_query::QueryReactionsResponse, msg::ReactionsMsg,
            querier::ReactionsQuerier,
        },
        relationships::{
            mocks::MockRelationshipsQueries, models_query::QueryRelationshipsResponse,
            msg::RelationshipsMsg, querier::RelationshipsQuerier,
        },
        reports::{
            mocks::MockReportsQueries, models_query::QueryReportResponse, msg::ReportsMsg,
            querier::ReportsQuerier,
        },
        subspaces::{
            mocks::MockSubspacesQueries, models_query::QuerySubspaceResponse, msg::SubspacesMsg,
            querier::SubspacesQuerier,
        },
    };
    use cw_multi_test::Executor;
    use std::ops::Deref;
    const SENDER: &str = "sender";

    #[test]
    fn execute_profiles_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Profiles(ProfilesMsg::delete_profile(Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_relationships_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Relationships(RelationshipsMsg::unblock_user(
                Addr::unchecked(SENDER),
                Addr::unchecked(SENDER),
                1,
            ))
            .into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_subspaces_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Subspaces(SubspacesMsg::delete_subspace(1, Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_posts_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Posts(PostsMsg::delete_post(1, 4, Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_reports_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Reports(ReportsMsg::delete_report(1, 1, Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn execute_reactions_msg_properly() {
        let mut app = mock_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Reactions(ReactionsMsg::remove_reaction(
                1,
                1,
                1,
                Addr::unchecked(SENDER),
            ))
            .into(),
        );
        assert!(result.is_ok());
    }
    #[test]
    fn test_profiles_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = ProfilesQuerier::new(app_querier.deref());
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };
        let response = querier.query_profile(Addr::unchecked("")).unwrap();
        assert_eq!(expected, response)
    }

    #[test]
    fn test_subspaces_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = SubspacesQuerier::new(app_querier.deref());
        let response = querier.query_subspace(1).unwrap();
        let expected = QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        };
        assert_eq!(expected, response);
    }

    #[test]
    fn test_relationships_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = RelationshipsQuerier::new(app_querier.deref());
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
    fn test_posts_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = PostsQuerier::new(app_querier.deref());
        let response = querier.query_post(1, 1).unwrap();
        let expected = QueryPostResponse {
            post: MockPostsQueries::get_mocked_post(1u64.into(), 1u64.into()),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_reactions_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = ReactionsQuerier::new(app_querier.deref());
        let response = querier.query_reactions(1, 1, None, None).unwrap();
        let expected = QueryReactionsResponse {
            reactions: vec![MockReactionsQueries::get_mock_reaction()],
            pagination: Default::default(),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn test_reports_query_properly() {
        let app = mock_desmos_app();
        let app_querier = app.wrap();
        let querier = ReportsQuerier::new(app_querier.deref());
        let response = querier.query_report(1, 1).unwrap();
        let expected = QueryReportResponse {
            report: MockReportsQueries::get_mocked_report(&1u64.into()),
        };
        assert_eq!(expected, response)
    }

    #[test]
    fn failing_app_excute_error() {
        let mut app = mock_failing_desmos_app();
        let result = app.execute(
            Addr::unchecked(SENDER),
            DesmosMsg::Profiles(ProfilesMsg::delete_profile(Addr::unchecked(SENDER))).into(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn failing_app_query_error() {
        let app = mock_failing_desmos_app();
        let app_querier = app.wrap();
        let querier = ProfilesQuerier::new(app_querier.deref());
        let result = querier.query_profile(Addr::unchecked(""));
        assert!(result.is_err())
    }
}
