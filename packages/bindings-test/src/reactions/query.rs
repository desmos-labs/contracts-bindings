#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, USER1_ADDRESS, USER2_ADDRESS};
    use cosmwasm_std::Addr;
    use desmos_bindings::reactions::models_query::{
        QueryReactionsResponse, QueryReactionResponse,
        QueryRegisteredReactionsResponse, QueryRegisteredReactionResponse,
        QueryReactionsParamsResponse,
    };
    use desmos_bindings::reactions::query::ReactionsQuery;
    use test_contract::msg::QueryMsg::DesmosChain;
}