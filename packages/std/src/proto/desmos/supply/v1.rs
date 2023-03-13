/// QueryTotalRequest is the request type for Query/Total RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.supply.v1.QueryTotalRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.supply.v1.Query/Total",
    response_type = QueryTotalResponse
)]
pub struct QueryTotalRequest {
    /// coin denom to query the circulating supply for
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// divider_exponent is a factor used to power the divider used to convert the
    /// supply to the desired representation
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub divider_exponent: u64,
}
/// QueryTotalResponse is the response type for the Query/Total RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.supply.v1.QueryTotalResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryTotalResponse {
    #[prost(string, tag = "1")]
    pub total_supply: ::prost::alloc::string::String,
}
/// QueryCirculatingRequest is the request type for the Query/Circulating RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.supply.v1.QueryCirculatingRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.supply.v1.Query/Circulating",
    response_type = QueryCirculatingResponse
)]
pub struct QueryCirculatingRequest {
    /// coin denom to query the circulating supply for
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// divider_exponent is a factor used to power the divider used to convert the
    /// supply to the desired representation
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub divider_exponent: u64,
}
/// QueryCirculatingResponse is the response type for the Query/Circulating RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.supply.v1.QueryCirculatingResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryCirculatingResponse {
    #[prost(string, tag = "1")]
    pub circulating_supply: ::prost::alloc::string::String,
}
pub struct SupplyQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> SupplyQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn total(
        &self,
        denom: ::prost::alloc::string::String,
        divider_exponent: u64,
    ) -> std::result::Result<QueryTotalResponse, cosmwasm_std::StdError> {
        QueryTotalRequest {
            denom,
            divider_exponent,
        }
        .query(self.querier)
    }
    pub fn circulating(
        &self,
        denom: ::prost::alloc::string::String,
        divider_exponent: u64,
    ) -> std::result::Result<QueryCirculatingResponse, cosmwasm_std::StdError> {
        QueryCirculatingRequest {
            denom,
            divider_exponent,
        }
        .query(self.querier)
    }
}
