use crate::msg::DesmosMsg;
use crate::query::DesmosQuery;
use anyhow::Result as AnyResult;
use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{Addr, Api, Binary, BlockInfo, Empty, Querier, Storage};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, Module, WasmKeeper,
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

pub type DesmosApp =
    App<BankKeeper, MockApi, MockStorage, DesmosKeeper, WasmKeeper<DesmosMsg, DesmosQuery>>;

#[derive(Default)]
pub struct DesmosKeeper {}

impl DesmosKeeper {
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

pub fn mock_desmos_app() -> DesmosApp {
    BasicAppBuilder::<DesmosMsg, DesmosQuery>::new_custom()
        .with_custom(DesmosKeeper::new())
        .build(|_, _, _| {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::msg::ProfilesMsg;
    use cw_multi_test::Executor;

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
}
