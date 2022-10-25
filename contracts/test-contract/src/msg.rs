use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CosmosMsg, QueryRequest};
use desmos_bindings::msg::DesmosMsg;
use desmos_bindings::query::DesmosQuery;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    DesmosMessages { msgs: Vec<CosmosMsg<DesmosMsg>> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ChainResponse)]
    DesmosChain { request: QueryRequest<DesmosQuery> },
}

#[cw_serde]
pub struct ChainResponse {
    pub data: Binary,
}
