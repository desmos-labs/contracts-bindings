/// ChainLinkJSON contains the data required to create a ChainLink using the CLI
/// command
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.client.ChainLinkJSON")]
#[serde(rename_all = "snake_case")]
pub struct ChainLinkJson {
    /// Address contains the data of the external chain address to be connected
    /// with the Desmos profile
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<crate::shim::Any>,
    /// Proof contains the ownership proof of the external chain address
    #[prost(message, optional, tag = "2")]
    pub proof: ::core::option::Option<super::Proof>,
    /// ChainConfig contains the configuration of the external chain
    #[prost(message, optional, tag = "3")]
    pub chain_config: ::core::option::Option<super::ChainConfig>,
}
