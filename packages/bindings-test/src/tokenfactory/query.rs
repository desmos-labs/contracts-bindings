#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_CREATION_DENOM_FEES_AMOUNT, TEST_CREATION_DENOM_FEES_DENOM};
    use crate::subspaces::create_test_subspace;
    use crate::tokenfactory::get_denom;
    use test_contract::msg::QueryMsg::DesmosChain;

    use cosmwasm_std::{Addr, Coin};
    use desmos_bindings::tokenfactory::msg::TokenfactoryMsg;
    use desmos_bindings::tokenfactory::types::{
        QuerySubspaceDenomsRequest, QuerySubspaceDenomsResponse,
    };

    #[test]
    fn test_query_subspace_denoms() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);
        let subdenom = "test_query";

        // Setup a subspace and denom for testing
        let subspace = create_test_subspace(&contract_address);

        desmos_cli
            .send_tokens(
                &subspace.treasury,
                Coin::new(
                    TEST_CREATION_DENOM_FEES_AMOUNT,
                    TEST_CREATION_DENOM_FEES_DENOM,
                ),
            )
            .assert_success();

        let create_denom_msg = TokenfactoryMsg::create_denom(
            subspace.id,
            Addr::unchecked(&contract_address),
            subdenom,
        );

        desmos_cli
            .execute_contract(&contract_address, vec![create_denom_msg.into()])
            .assert_success();

        // Query subspace denoms
        let query = DesmosChain {
            request: QuerySubspaceDenomsRequest {
                subspace_id: subspace.id,
            }
            .into(),
        };
        let response: QuerySubspaceDenomsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        assert_eq!(vec![get_denom(&subspace, subdenom)], response.denoms)
    }
}
