/// BasicAllowance implements Allowance with a one-time grant of coins
/// that optionally expires. The grantee can use up to SpendLimit to cover fees.
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
#[proto_message(type_url = "/cosmos.feegrant.v1beta1.BasicAllowance")]
#[serde(rename_all = "snake_case")]
pub struct BasicAllowance {
    /// spend_limit specifies the maximum amount of coins that can be spent
    /// by this allowance and will be updated as coins are spent. If it is
    /// empty, there is no spend limit and any amount of coins can be spent.
    #[prost(message, repeated, tag = "1")]
    pub spend_limit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// expiration specifies an optional time when this allowance expires
    #[prost(message, optional, tag = "2")]
    pub expiration: ::core::option::Option<crate::shim::Timestamp>,
}
/// PeriodicAllowance extends Allowance to allow for both a maximum cap,
/// as well as a limit per time period.
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
#[proto_message(type_url = "/cosmos.feegrant.v1beta1.PeriodicAllowance")]
#[serde(rename_all = "snake_case")]
pub struct PeriodicAllowance {
    /// basic specifies a struct of `BasicAllowance`
    #[prost(message, optional, tag = "1")]
    pub basic: ::core::option::Option<BasicAllowance>,
    /// period specifies the time duration in which period_spend_limit coins can
    /// be spent before that allowance is reset
    #[prost(message, optional, tag = "2")]
    pub period: ::core::option::Option<crate::shim::Duration>,
    /// period_spend_limit specifies the maximum number of coins that can be spent
    /// in the period
    #[prost(message, repeated, tag = "3")]
    pub period_spend_limit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// period_can_spend is the number of coins left to be spent before the period_reset time
    #[prost(message, repeated, tag = "4")]
    pub period_can_spend: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// period_reset is the time at which this period resets and a new one begins,
    /// it is calculated from the start time of the first transaction after the
    /// last period ended
    #[prost(message, optional, tag = "5")]
    pub period_reset: ::core::option::Option<crate::shim::Timestamp>,
}
/// AllowedMsgAllowance creates allowance only for specified message types.
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
#[proto_message(type_url = "/cosmos.feegrant.v1beta1.AllowedMsgAllowance")]
#[serde(rename_all = "snake_case")]
pub struct AllowedMsgAllowance {
    /// allowance can be any of basic and periodic fee allowance.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<crate::shim::Any>,
    /// allowed_messages are the messages for which the grantee has the access.
    #[prost(string, repeated, tag = "2")]
    pub allowed_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Grant is stored in the KVStore to record a grant with full context
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
#[proto_message(type_url = "/cosmos.feegrant.v1beta1.Grant")]
#[serde(rename_all = "snake_case")]
pub struct Grant {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic, periodic, allowed fee allowance.
    #[prost(message, optional, tag = "3")]
    pub allowance: ::core::option::Option<crate::shim::Any>,
}
