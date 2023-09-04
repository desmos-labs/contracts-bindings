/// Params defines the parameters for the tokenfactory module.
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    /// DenomCreationFee defines the fee to be charged on the creation of a new
    /// denom. The fee is drawn from the subspace treasury account, and
    /// burned.
    #[prost(message, repeated, tag = "1")]
    pub denom_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// DenomAuthorityMetadata contains the metadata for a single token denom.
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.DenomAuthorityMetadata")]
#[serde(rename_all = "snake_case")]
pub struct DenomAuthorityMetadata {
    /// Admin of the denomination.
    /// Can be empty for no admin, or a valid Desmos address
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
}
/// GenesisState defines the tokenfactory module's genesis state.
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.GenesisState")]
#[serde(rename_all = "snake_case")]
pub struct GenesisState {
    /// params defines the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub factory_denoms: ::prost::alloc::vec::Vec<GenesisDenom>,
}
/// GenesisDenom defines a tokenfactory denom that is defined within genesis
/// state. The structure contains DenomAuthorityMetadata which defines the
/// denom's admin.
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.GenesisDenom")]
#[serde(rename_all = "snake_case")]
pub struct GenesisDenom {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// MsgCreateDenom represents the message to be used to create a denom for
/// subspace
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgCreateDenom")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateDenom {
    /// Id of the subspace which manages the denom
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of user having the permission to manage subspace denoms
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// Subdenom name of the creating denom
    /// It can be up to 44 "alphanumeric" characters long
    #[prost(string, tag = "3")]
    pub subdenom: ::prost::alloc::string::String,
}
/// MsgCreateDenomResponse represents the Msg/CreateDenom response type
/// It returns the full string of the newly created denom
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgCreateDenomResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateDenomResponse {
    /// Name of the newly created denom
    #[prost(string, tag = "1")]
    pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint represents the message to be used to mint subspace tokens to treasury
/// account
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgMint")]
#[serde(rename_all = "snake_case")]
pub struct MsgMint {
    /// Id of the subspace which manages the denom
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of user having the permission to manage subspace denoms
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// Amount of the minting subspace tokens
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgMintResponse represents the Msg/Mint response type
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgMintResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMintResponse {}
/// MsgBurn represents the message to be used to burn subspace tokens from
/// treasury account
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgBurn")]
#[serde(rename_all = "snake_case")]
pub struct MsgBurn {
    /// Id of the subspace which manages the denom
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of user having the permission to manage subspace denoms
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// Amount of the burning subspace tokens
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgBurnResponse represents the Msg/Burn response type
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgBurnResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgBurnResponse {}
/// MsgSetDenomMetadata represents the message to be used to set the subspace
/// token's bank metadata
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgSetDenomMetadata")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetDenomMetadata {
    /// Id of the subspace which manages the denom
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of user having the permission to manage subspace denoms
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// Metadata of the denom
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// MsgSetDenomMetadataResponse represents the Msg/SetDenomMetadata response type
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgSetDenomMetadataResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetDenomMetadataResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgUpdateParams")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse represents the Msg/UpdateParams response type
///
/// Since: Desmos 6.0.0
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.MsgUpdateParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateParamsResponse {}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.tokenfactory.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.QueryParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QuerySubspaceDenomsRequest defines the request structure for the
/// SubspaceDenoms gRPC query.
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.QuerySubspaceDenomsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.tokenfactory.v1.Query/SubspaceDenoms",
    response_type = QuerySubspaceDenomsResponse
)]
pub struct QuerySubspaceDenomsRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// QuerySubspaceDenomsResponse defines the response structure for the
/// SubspaceDenoms gRPC query.
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
#[proto_message(type_url = "/desmos.tokenfactory.v1.QuerySubspaceDenomsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspaceDenomsResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
pub struct TokenfactoryQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TokenfactoryQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn subspace_denoms(
        &self,
        subspace_id: u64,
    ) -> std::result::Result<QuerySubspaceDenomsResponse, cosmwasm_std::StdError> {
        QuerySubspaceDenomsRequest { subspace_id }.query(self.querier)
    }
}
