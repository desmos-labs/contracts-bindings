mod msg;
mod query;

use crate::chain_communication::DesmosCli;

use cosmwasm_std::Addr;

use test_contract::msg::QueryMsg::DesmosChain;

use desmos_bindings::cosmos_types::PageRequest;
use desmos_bindings::subspaces::msg::SubspacesMsg;
use desmos_bindings::subspaces::types::{QuerySubspacesRequest, QuerySubspacesResponse, Subspace};

/// Creates a subspace for interaction testing
pub fn create_test_subspace(contract_address: &str) -> Subspace {
    let desmos_cli = DesmosCli::default();

    let create_subspace =
        SubspacesMsg::create_subspace("test", "test", Addr::unchecked(contract_address), Addr::unchecked(contract_address));

    desmos_cli
        .execute_contract(contract_address, vec![create_subspace.into()])
        .assert_success();

    // query the created subspace
    let result: QuerySubspacesResponse = desmos_cli
        .wasm_query(
            contract_address,
            &DesmosChain {
                request: QuerySubspacesRequest {
                    pagination: Some(PageRequest {
                        key: vec![],
                        limit: 1,
                        offset: 0,
                        count_total: false,
                        reverse: true,
                    }),
                }
                .into(),
            },
        )
        .to_object();

    result.subspaces.first().unwrap().clone()
}
