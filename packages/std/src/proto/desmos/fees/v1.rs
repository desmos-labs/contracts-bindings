/// MinFee contains the minimum amount of coins that should be paid as a fee for
/// each message of the specific type sent
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.fees.v1.MinFee")]
pub struct MinFee {
    #[prost(string, tag = "1")]
    pub message_type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Params contains the parameters for the fees module
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.fees.v1.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub min_fees: ::prost::alloc::vec::Vec<MinFee>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.fees.v1.QueryParamsRequest")]
#[proto_query(
    path = "/desmos.fees.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.fees.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
pub struct FeesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}