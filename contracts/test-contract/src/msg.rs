use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CosmosMsg, Empty, QueryRequest};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    DesmosMessages { msgs: Vec<CosmosMsg> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ChainResponse)]
    DesmosChain { request: QueryRequest<Empty> },
}

#[cw_serde]
pub struct ChainResponse {
    pub data: Binary,
}
