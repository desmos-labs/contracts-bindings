use crate::msg::DesmosMsg;
use crate::query::DesmosQuery;
use anyhow::Result as AnyResult;
use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{Addr, Api, Binary, BlockInfo, Empty, Querier, Storage};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, FailingDistribution,
    FailingStaking, Module, Router, WasmKeeper,
};

#[cfg(feature = "posts")]
use crate::posts::mocks::mock_posts_query_response;
#[cfg(feature = "profiles")]
use crate::profiles::mocks::mock_profiles_query_response;
#[cfg(feature = "reactions")]
use crate::reactions::mocks::mock_reactions_query_response;
#[cfg(feature = "relationships")]
use crate::relationships::mocks::mock_relationships_query_response;
#[cfg(feature = "reports")]
use crate::reports::mocks::mock_reports_query_response;
#[cfg(feature = "subspaces")]
use crate::subspaces::mocks::mock_subspaces_query_response;

/// DesmosApp wraps the desmos custom module into a mock app for integration tests.
pub type DesmosApp =
    App<BankKeeper, MockApi, MockStorage, DesmosKeeper, WasmKeeper<DesmosMsg, DesmosQuery>>;

/// Represents the implementation of [`Module`](cw_multi_test::Module) for handling the desmos execution and query messages.
#[derive(Default)]
pub struct DesmosKeeper {}

impl DesmosKeeper {
    /// Returns a new [DesmosKeeper].
    pub fn new() -> Self {
        DesmosKeeper {}
    }
}

impl Module for DesmosKeeper {
    type ExecT = DesmosMsg;
    type QueryT = DesmosQuery;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        msg: DesmosMsg,
    ) -> AnyResult<AppResponse> {
        match msg {
            DesmosMsg::Profiles(_) => AnyResult::Ok(AppResponse::default()),
            DesmosMsg::Subspaces(_) => AnyResult::Ok(AppResponse::default()),
            DesmosMsg::Relationships(_) => AnyResult::Ok(AppResponse::default()),
            DesmosMsg::Posts(_) => AnyResult::Ok(AppResponse::default()),
            DesmosMsg::Reports(_) => AnyResult::Ok(AppResponse::default()),
            DesmosMsg::Reactions(_) => AnyResult::Ok(AppResponse::default()),
        }
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: DesmosQuery,
    ) -> AnyResult<Binary> {
        match request {
            #[cfg(feature = "profiles")]
            DesmosQuery::Profiles(query) => {
                AnyResult::Ok(mock_profiles_query_response(&query).unwrap())
            }
            #[cfg(feature = "subspaces")]
            DesmosQuery::Subspaces(query) => {
                AnyResult::Ok(mock_subspaces_query_response(&query).unwrap())
            }
            #[cfg(feature = "relationships")]
            DesmosQuery::Relationships(query) => {
                AnyResult::Ok(mock_relationships_query_response(&query).unwrap())
            }
            #[cfg(feature = "posts")]
            DesmosQuery::Posts(query) => AnyResult::Ok(mock_posts_query_response(&query).unwrap()),
            #[cfg(feature = "reactions")]
            DesmosQuery::Reactions(query) => {
                AnyResult::Ok(mock_reactions_query_response(&query).unwrap())
            }
            #[cfg(feature = "reports")]
            DesmosQuery::Reports(query) => {
                AnyResult::Ok(mock_reports_query_response(&query).unwrap())
            }
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Empty,
    ) -> AnyResult<AppResponse> {
        unimplemented!()
    }
}

/// Creates new default `DesmosApp` with customized initialization function.
pub fn custom_desmos_app<F>(init_fn: F) -> DesmosApp
where
    F: FnOnce(
        &mut Router<
            BankKeeper,
            DesmosKeeper,
            WasmKeeper<DesmosMsg, DesmosQuery>,
            FailingStaking,
            FailingDistribution,
        >,
        &dyn Api,
        &mut dyn Storage,
    ),
{
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom()
        .with_custom(DesmosKeeper::new())
        .build(init_fn)
}

/// Returns a mock desmos app
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
            mocks::get_mocked_post, models_query::QueryPostResponse, msg::PostsMsg,
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
            mocks::get_mocked_report, models_query::QueryReportResponse, msg::ReportsMsg,
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
            post: get_mocked_post(1u64.into(), 1u64.into()),
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
            report: get_mocked_report(&1u64.into()),
        };
        assert_eq!(expected, response)
    }
}
