use std_derive::CosmwasmExt;
/// QueryProfileRequest is the request type for the Query/Profile RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryProfileRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/Profile",
    response_type = QueryProfileResponse
)]
pub struct QueryProfileRequest {
    /// Address or DTag of the user to query the profile for
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
/// QueryProfileResponse is the response type for the Query/Profile RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryProfileResponse")]
pub struct QueryProfileResponse {
    #[prost(message, optional, tag = "1")]
    pub profile: ::core::option::Option<crate::shim::Any>,
}
/// DTagTransferRequest represent a DTag transfer request between two users
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.DTagTransferRequest")]
pub struct DTagTransferRequest {
    /// DTagToTrade contains the value of the DTag that should be transferred from
    /// the receiver of the request to the sender
    #[prost(string, tag = "1")]
    pub dtag_to_trade: ::prost::alloc::string::String,
    /// Sender represents the address of the account that sent the request
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// Receiver represents the receiver of the request that, if accepted, will
    /// give to the sender their DTag
    #[prost(string, tag = "3")]
    pub receiver: ::prost::alloc::string::String,
}
/// QueryIncomingDTagTransferRequestsRequest is the request type for the
/// Query/IncomingDTagTransferRequests RPC endpoint
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryIncomingDTagTransferRequestsRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/IncomingDTagTransferRequests",
    response_type = QueryIncomingDTagTransferRequestsResponse
)]
pub struct QueryIncomingDTagTransferRequestsRequest {
    /// (optional) Receiver represents the address of the user to which query the
    /// incoming requests for
    #[prost(string, tag = "1")]
    pub receiver: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryIncomingDTagTransferRequestsResponse is the response type for the
/// Query/IncomingDTagTransferRequests RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryIncomingDTagTransferRequestsResponse")]
pub struct QueryIncomingDTagTransferRequestsResponse {
    /// Requests represent the list of all the DTag transfer requests made towards
    /// the user
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DTagTransferRequest>,
    /// Pagination defines the pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Params contains the parameters for the profiles module
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub nickname: ::core::option::Option<NicknameParams>,
    #[prost(message, optional, tag = "2")]
    pub dtag: ::core::option::Option<DTagParams>,
    #[prost(message, optional, tag = "3")]
    pub bio: ::core::option::Option<BioParams>,
    #[prost(message, optional, tag = "4")]
    pub oracle: ::core::option::Option<OracleParams>,
    #[prost(message, optional, tag = "5")]
    pub app_links: ::core::option::Option<AppLinksParams>,
}
/// NicknameParams defines the parameters related to the profiles nicknames
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.NicknameParams")]
pub struct NicknameParams {
    #[prost(bytes = "vec", tag = "1")]
    pub min_length: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub max_length: ::prost::alloc::vec::Vec<u8>,
}
/// DTagParams defines the parameters related to profile DTags
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.DTagParams")]
pub struct DTagParams {
    #[prost(string, tag = "1")]
    pub reg_ex: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub min_length: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub max_length: ::prost::alloc::vec::Vec<u8>,
}
/// BioParams defines the parameters related to profile biography
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.BioParams")]
pub struct BioParams {
    #[prost(bytes = "vec", tag = "3")]
    pub max_length: ::prost::alloc::vec::Vec<u8>,
}
/// OracleParams defines the parameters related to the oracle
/// that will be used to verify the ownership of a centralized
/// application account by a Desmos profile
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.OracleParams")]
pub struct OracleParams {
    /// ScriptID represents the ID of the oracle script to be called to verify the
    /// data
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub script_id: u64,
    /// AskCount represents the number of oracles to which ask to verify the data
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ask_count: u64,
    /// MinCount represents the minimum count of oracles that should complete the
    /// verification successfully
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_count: u64,
    /// PrepareGas represents the amount of gas to be used during the preparation
    /// stage of the oracle script
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub prepare_gas: u64,
    /// ExecuteGas represents the amount of gas to be used during the execution of
    /// the oracle script
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub execute_gas: u64,
    /// FeeAmount represents the amount of fees to be payed in order to execute the
    /// oracle script
    #[prost(message, repeated, tag = "6")]
    pub fee_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// AppLinksParams define the parameters related to the app links
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.AppLinksParams")]
pub struct AppLinksParams {
    /// Default validity duration before an application link expires
    #[prost(message, optional, tag = "1")]
    pub validity_duration: ::core::option::Option<crate::shim::Duration>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC endpoint
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryParamsRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// ChainLink contains the data representing either an inter- or cross- chain
/// link
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.ChainLink")]
pub struct ChainLink {
    /// User defines the destination profile address to link
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// Address contains the data of the external chain address to be connected
    /// with the Desmos profile
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<crate::shim::Any>,
    /// Proof contains the ownership proof of the external chain address
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<Proof>,
    /// ChainConfig contains the configuration of the external chain
    #[prost(message, optional, tag = "4")]
    pub chain_config: ::core::option::Option<ChainConfig>,
    /// CreationTime represents the time in which the link has been created
    #[prost(message, optional, tag = "5")]
    pub creation_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// ChainConfig contains the data of the chain with which the link is made.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.ChainConfig")]
pub struct ChainConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Proof contains all the data used to verify a signature when linking an
/// account to a profile
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Proof")]
pub struct Proof {
    /// PubKey represents the public key associated with the address for which to
    /// prove the ownership
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<crate::shim::Any>,
    /// Signature represents the hex-encoded signature of the PlainText value
    #[prost(message, optional, tag = "2")]
    pub signature: ::core::option::Option<crate::shim::Any>,
    /// PlainText represents the hex-encoded value signed in order to produce the
    /// Signature
    #[prost(string, tag = "3")]
    pub plain_text: ::prost::alloc::string::String,
}
/// Bech32Address represents a Bech32-encoded address
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Bech32Address")]
pub struct Bech32Address {
    /// Value represents the Bech-32 encoded address value
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// Prefix represents the HRP of the Bech32 address
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
}
/// Base58Address represents a Base58-encoded address
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Base58Address")]
pub struct Base58Address {
    /// Value contains the Base58-encoded address
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// HexAddress represents an Hex-encoded address
/// NOTE: Currently it only supports keccak256-uncompressed addresses
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.HexAddress")]
pub struct HexAddress {
    /// Value represents the hex address value
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// Prefix represents the optional prefix used during address encoding (e.g.
    /// 0x)
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
}
/// SingleSignature is the signature data for a single signer
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.SingleSignature")]
pub struct SingleSignature {
    /// Type represents the type of the signature value
    #[prost(enumeration = "SignatureValueType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub value_type: i32,
    /// Signature is the raw signature bytes
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// CosmosMultiSignature is the signature data for a multisig public key
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.CosmosMultiSignature")]
pub struct CosmosMultiSignature {
    /// Bitarray specifies which keys within the multisig are signing
    #[prost(message, optional, tag = "1")]
    pub bit_array: ::core::option::Option<
        super::super::super::cosmos::crypto::multisig::v1beta1::CompactBitArray,
    >,
    /// Signatures is the signatures of the multi-signature
    #[prost(message, repeated, tag = "2")]
    pub signatures: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// SignatureValueType specifies all the possible signature types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum SignatureValueType {
    /// SIGNATURE_VALUE_TYPE_UNSPECIFIED specifies an unknown signing mode
    /// and will be rejected
    Unspecified = 0,
    /// SIGNATURE_VALUE_TYPE_RAW should be used when the value has been
    /// signed as a raw byte array
    Raw = 1,
    /// SIGNATURE_VALUE_TYPE_COSMOS_DIRECT should be used when the signed
    /// value has been encoded as a Protobuf transaction containing the owner
    /// address inside its memo field
    CosmosDirect = 2,
    /// SIGNATURE_VALUE_TYPE_COSMOS_AMINO should be used when the value has
    /// been encoded as an Amino transaction containing the owner address inside
    /// its memo field
    CosmosAmino = 3,
    /// SIGNATURE_VALUE_TYPE_EVM_PERSONAL_SIGN should be used when the value
    /// has been encoded following the EVM personal_sign specification
    EvmPersonalSign = 4,
}
/// QueryChainLinksRequest represents the request that should be used in order
/// to retrieve the link associated with the provided user, for the given chain
/// and having the given target address
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryChainLinksRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/ChainLinks",
    response_type = QueryChainLinksResponse
)]
pub struct QueryChainLinksRequest {
    /// (optional) User represents the Desmos address of the user to which search
    /// the link for
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// (optional) ChainName contains the name of the chain to which search the
    /// link for. Used only if user is also set
    #[prost(string, tag = "2")]
    pub chain_name: ::prost::alloc::string::String,
    /// (optional) Target must contain the external address to which query the link
    /// for. Used only if chain name is also set
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryChainLinksResponse is the response type for the
/// Query/ChainLinks RPC method.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryChainLinksResponse")]
pub struct QueryChainLinksResponse {
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<ChainLink>,
    /// Pagination defines the pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryChainLinkOwnersRequest contains the data of the request that can
/// be used to get chain link owners
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryChainLinkOwnersRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/ChainLinkOwners",
    response_type = QueryChainLinkOwnersResponse
)]
pub struct QueryChainLinkOwnersRequest {
    /// (Optional) Chain name to search link owners of. If not specified, all
    /// links stored will be searched instead.
    #[prost(string, tag = "1")]
    pub chain_name: ::prost::alloc::string::String,
    /// (Optional) External address to search for. This will only be used if the
    /// chain name is specified as well
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryChainLinkOwnersResponse contains the data returned by the request
/// allowing to get chain link owners.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryChainLinkOwnersResponse")]
pub struct QueryChainLinkOwnersResponse {
    /// Addresses of the chain links owners
    #[prost(message, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<query_chain_link_owners_response::ChainLinkOwnerDetails>,
    /// Pagination defines the pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Nested message and enum types in `QueryChainLinkOwnersResponse`.
pub mod query_chain_link_owners_response {
    use std_derive::CosmwasmExt;
    /// ChainLinkOwnerDetails contains the details of a single chain link owner
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        CosmwasmExt,
    )]
    #[proto_message(
        type_url = "/desmos.profiles.v3.QueryChainLinkOwnersResponse.ChainLinkOwnerDetails"
    )]
    pub struct ChainLinkOwnerDetails {
        #[prost(string, tag = "1")]
        pub user: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub chain_name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub target: ::prost::alloc::string::String,
    }
}
/// QueryDefaultExternalAddressesRequest is the request type for
/// Query/DefaultExternalAddresses RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryDefaultExternalAddressesRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/DefaultExternalAddresses",
    response_type = QueryDefaultExternalAddressesResponse
)]
pub struct QueryDefaultExternalAddressesRequest {
    /// (Optional) Owner for which to query the default addresses
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// (Optional) Chain name to query the default address for
    #[prost(string, tag = "2")]
    pub chain_name: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDefaultExternalAddressesResponse is the response type for
/// Query/DefaultExternalAddresses RPC method
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryDefaultExternalAddressesResponse")]
pub struct QueryDefaultExternalAddressesResponse {
    /// List of default addresses, each one represented by the associated chain
    /// link
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<ChainLink>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// ApplicationLink contains the data of a link to a centralized application
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.ApplicationLink")]
pub struct ApplicationLink {
    ///  User to which the link is associated
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// Data contains the details of this specific link
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    /// State of the link
    #[prost(enumeration = "ApplicationLinkState", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub state: i32,
    /// OracleRequest represents the request that has been made to the oracle
    #[prost(message, optional, tag = "4")]
    pub oracle_request: ::core::option::Option<OracleRequest>,
    /// Data coming from the result of the verification.
    /// Only available when the state is STATE_SUCCESS
    #[prost(message, optional, tag = "5")]
    pub result: ::core::option::Option<AppResult>,
    /// CreationTime represents the time in which the link was created
    #[prost(message, optional, tag = "6")]
    pub creation_time: ::core::option::Option<crate::shim::Timestamp>,
    /// ExpirationTime represents the time in which the link will expire
    #[prost(message, optional, tag = "7")]
    pub expiration_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// Data contains the data associated to a specific user of a
/// generic centralized application
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Data")]
pub struct Data {
    /// The application name (eg. Twitter, GitHub, etc)
    #[prost(string, tag = "1")]
    pub application: ::prost::alloc::string::String,
    /// Username on the application (eg. Twitter tag, GitHub profile, etc)
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
/// OracleRequest represents a generic oracle request used to
/// verify the ownership of a centralized application account
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.OracleRequest")]
pub struct OracleRequest {
    /// ID is the ID of the request
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// OracleScriptID is ID of an oracle script
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub oracle_script_id: u64,
    /// CallData contains the data used to perform the oracle request
    #[prost(message, optional, tag = "3")]
    pub call_data: ::core::option::Option<oracle_request::CallData>,
    /// ClientID represents the ID of the client that has called the oracle script
    #[prost(string, tag = "4")]
    pub client_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OracleRequest`.
pub mod oracle_request {
    use std_derive::CosmwasmExt;
    /// CallData contains the data sent to a single oracle request in order to
    /// verify the ownership of a centralized application by a Desmos profile
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        CosmwasmExt,
    )]
    #[proto_message(type_url = "/desmos.profiles.v3.OracleRequest.CallData")]
    pub struct CallData {
        /// The application for which the ownership should be verified
        #[prost(string, tag = "1")]
        pub application: ::prost::alloc::string::String,
        /// The hex encoded call data that should be used to verify the application
        /// account ownership
        #[prost(string, tag = "2")]
        pub call_data: ::prost::alloc::string::String,
    }
}
/// Result represents a verification result
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Result")]
pub struct AppResult {
    /// sum is the oneof that specifies whether this represents a success or
    /// failure result
    #[prost(oneof = "result::Sum", tags = "1, 2")]
    pub sum: ::core::option::Option<result::Sum>,
}
/// Nested message and enum types in `Result`.
pub mod result {
    use std_derive::CosmwasmExt;
    /// Success is the result of an application link that has been successfully
    /// verified
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        CosmwasmExt,
    )]
    #[proto_message(type_url = "/desmos.profiles.v3.Result.Success")]
    pub struct Success {
        /// Hex-encoded value that has be signed by the profile
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
        /// Hex-encoded signature that has been produced by signing the value
        #[prost(string, tag = "2")]
        pub signature: ::prost::alloc::string::String,
    }
    /// Failed is the result of an application link that has not been verified
    /// successfully
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        CosmwasmExt,
    )]
    #[proto_message(type_url = "/desmos.profiles.v3.Result.Failed")]
    pub struct Failed {
        /// Error that is associated with the failure
        #[prost(string, tag = "1")]
        pub error: ::prost::alloc::string::String,
    }
    /// sum is the oneof that specifies whether this represents a success or
    /// failure result
    #[derive(
        Clone, PartialEq, ::prost::Oneof, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
    )]
    pub enum Sum {
        /// Success represents a successful verification
        #[prost(message, tag = "1")]
        Success(Success),
        /// Failed represents a failed verification
        #[prost(message, tag = "2")]
        Failed(Failed),
    }
}
/// ApplicationLinkState defines if an application link is in the following
/// states: STARTED, ERRORED, SUCCESSFUL, TIMED_OUT
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum ApplicationLinkState {
    /// A link has just been initialized
    InitializedUnspecified = 0,
    /// A link has just started being verified
    VerificationStarted = 1,
    /// A link has errored during the verification process
    VerificationError = 2,
    /// A link has being verified successfully
    VerificationSuccess = 3,
    /// A link has timed out while waiting for the verification
    TimedOut = 4,
}
/// QueryUserApplicationLinkRequest represents the request used when querying an
/// application link using an application name and username for a given user
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryApplicationLinksRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/ApplicationLinks",
    response_type = QueryApplicationLinksResponse
)]
pub struct QueryApplicationLinksRequest {
    /// (Optional) User contains the Desmos profile address associated for which
    /// the link should be searched for
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// (Optional) Application represents the application name associated with the
    /// link. Used only if user is also set.
    #[prost(string, tag = "2")]
    pub application: ::prost::alloc::string::String,
    /// Username represents the username inside the application associated with the
    /// link. Used only if application is also set.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryApplicationLinksResponse represents the response to the query used
/// to get the application links for a specific user
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryApplicationLinksResponse")]
pub struct QueryApplicationLinksResponse {
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<ApplicationLink>,
    /// Pagination defines the pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryApplicationLinkByClientIDRequest contains the data of the request that
/// can be used to get an application link based on a client id
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryApplicationLinkByClientIDRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/ApplicationLinkByClientID",
    response_type = QueryApplicationLinkByClientIdResponse
)]
pub struct QueryApplicationLinkByClientIdRequest {
    /// ClientID represents the ID of the client to which search the link for
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryApplicationLinkByClientIDResponse contains the data returned by the
/// request allowing to get an application link using a client id
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryApplicationLinkByClientIDResponse")]
pub struct QueryApplicationLinkByClientIdResponse {
    #[prost(message, optional, tag = "1")]
    pub link: ::core::option::Option<ApplicationLink>,
}
/// QueryApplicationLinkOwnersRequest contains the data of the request that can
/// be used to get application link owners
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryApplicationLinkOwnersRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/ApplicationLinkOwners",
    response_type = QueryApplicationLinkOwnersResponse
)]
pub struct QueryApplicationLinkOwnersRequest {
    /// (Optional) Application name to search link owners of. If not specified, all
    /// links stored will be searched instead.
    #[prost(string, tag = "1")]
    pub application: ::prost::alloc::string::String,
    /// (Optional) Username to search for. This will only be used if the
    /// application is specified as well
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryApplicationLinkOwnersResponse contains the data returned by the request
/// allowing to get application link owners.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryApplicationLinkOwnersResponse")]
pub struct QueryApplicationLinkOwnersResponse {
    /// Addresses of the application links owners
    #[prost(message, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<
        query_application_link_owners_response::ApplicationLinkOwnerDetails,
    >,
    /// Pagination defines the pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Nested message and enum types in `QueryApplicationLinkOwnersResponse`.
pub mod query_application_link_owners_response {
    use std_derive::CosmwasmExt;
    /// ApplicationLinkOwnerDetails contains the details of a single application
    /// link owner
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
        CosmwasmExt,
    )]
    #[proto_message(
        type_url = "/desmos.profiles.v3.QueryApplicationLinkOwnersResponse.ApplicationLinkOwnerDetails"
    )]
    pub struct ApplicationLinkOwnerDetails {
        #[prost(string, tag = "1")]
        pub user: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub application: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub username: ::prost::alloc::string::String,
    }
}
/// Profile represents a generic first on Desmos, containing the information of a
/// single user
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Profile")]
pub struct Profile {
    /// Account represents the base Cosmos account associated with this profile
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<crate::shim::Any>,
    /// DTag represents the unique tag of this profile
    #[prost(string, tag = "2")]
    pub dtag: ::prost::alloc::string::String,
    /// Nickname contains the custom human readable name of the profile
    #[prost(string, tag = "3")]
    pub nickname: ::prost::alloc::string::String,
    /// Bio contains the biography of the profile
    #[prost(string, tag = "4")]
    pub bio: ::prost::alloc::string::String,
    /// Pictures contains the data about the pictures associated with he profile
    #[prost(message, optional, tag = "5")]
    pub pictures: ::core::option::Option<Pictures>,
    /// CreationTime represents the time in which the profile has been created
    #[prost(message, optional, tag = "6")]
    pub creation_date: ::core::option::Option<crate::shim::Timestamp>,
}
/// Pictures contains the data of a user profile's related pictures
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.Pictures")]
pub struct Pictures {
    /// Profile contains the URL to the profile picture
    #[prost(string, tag = "1")]
    pub profile: ::prost::alloc::string::String,
    /// Cover contains the URL to the cover picture
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
}
/// MsgRequestDTagTransfer represents the message used to request the DTag
/// transfer to another user.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgRequestDTagTransfer")]
pub struct MsgRequestDTagTransfer {
    /// Receiver contains the address of the request receiver that owns the DTag to
    /// transfer if the request is accepted
    #[prost(string, tag = "1")]
    pub receiver: ::prost::alloc::string::String,
    /// Sender contains the address of the request sender that will receive the
    /// receiver DTag if the requet is accepted
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgRequestDTagTransferResponse defines the Msg/RequestDTagTransfer response
/// type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgRequestDTagTransferResponse")]
pub struct MsgRequestDTagTransferResponse {}
/// MsgCancelDTagTransferRequest represents the message used to cancel a DTag
/// transfer request.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgCancelDTagTransferRequest")]
pub struct MsgCancelDTagTransferRequest {
    /// Receiver contains the address of the request receiver
    #[prost(string, tag = "1")]
    pub receiver: ::prost::alloc::string::String,
    /// Sender contains the address of the requets sender
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgCancelDTagTransferRequestResponse represents the
/// Msg/CancelDTagTransferRequest response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgCancelDTagTransferRequestResponse")]
pub struct MsgCancelDTagTransferRequestResponse {}
/// MsgAcceptDTagTransferRequest represents the message used to accept a DTag
/// transfer request.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgAcceptDTagTransferRequest")]
pub struct MsgAcceptDTagTransferRequest {
    /// NewDTag represents the DTag that the request receiver will obtain if they
    /// accept the request
    #[prost(string, tag = "1")]
    pub new_dtag: ::prost::alloc::string::String,
    /// Sender represents the request sender
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// Receiver represents the request receiver
    #[prost(string, tag = "3")]
    pub receiver: ::prost::alloc::string::String,
}
/// MsgAcceptDTagTransferRequestResponse defines the
/// Msg/AcceptDTagTransferRequest response.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgAcceptDTagTransferRequestResponse")]
pub struct MsgAcceptDTagTransferRequestResponse {}
/// MsgRefuseDTagTransferRequest represents the message used to refuse a DTag
/// transfer request.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgRefuseDTagTransferRequest")]
pub struct MsgRefuseDTagTransferRequest {
    /// Sender represents the request sender
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Receiver represents the request receiver
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
}
/// MsgRefuseDTagTransferRequestResponse defines the
/// Msg/RefuseDTagTransferRequest response.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgRefuseDTagTransferRequestResponse")]
pub struct MsgRefuseDTagTransferRequestResponse {}
/// MsgSaveProfile represents a message to save a profile.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgSaveProfile")]
pub struct MsgSaveProfile {
    /// DTag of the profile. If it shouldn't be changed, \[do-no-modify\] can be used
    /// instead.
    #[prost(string, tag = "1")]
    pub dtag: ::prost::alloc::string::String,
    /// Nickname of the profile. If it shouldn't be changed, \[do-no-modify\] can be
    /// used instead.
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
    /// Bio of the profile. If it shouldn't be changed, \[do-no-modify\] can be used
    /// instead.
    #[prost(string, tag = "3")]
    pub bio: ::prost::alloc::string::String,
    /// URL to the profile picture. If it shouldn't be changed, \[do-no-modify\] can
    /// be used instead.
    #[prost(string, tag = "4")]
    pub profile_picture: ::prost::alloc::string::String,
    /// URL to the profile cover. If it shouldn't be changed, \[do-no-modify\] can be
    /// used instead.
    #[prost(string, tag = "5")]
    pub cover_picture: ::prost::alloc::string::String,
    /// Address of the user associated to the profile
    #[prost(string, tag = "6")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgSaveProfileResponse defines the Msg/SaveProfile response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgSaveProfileResponse")]
pub struct MsgSaveProfileResponse {}
/// MsgDeleteProfile represents the message used to delete an existing profile.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgDeleteProfile")]
pub struct MsgDeleteProfile {
    /// Address associated to the profile to be deleted
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeleteProfileResponse defines the Msg/DeleteProfile response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgDeleteProfileResponse")]
pub struct MsgDeleteProfileResponse {}
/// MsgLinkChainAccount represents a message to link an account to a profile.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgLinkChainAccount")]
pub struct MsgLinkChainAccount {
    /// ChainAddress contains the details of the external chain address to be
    /// linked
    #[prost(message, optional, tag = "1")]
    pub chain_address: ::core::option::Option<crate::shim::Any>,
    /// Proof contains the proof of ownership of the external chain address
    #[prost(message, optional, tag = "2")]
    pub proof: ::core::option::Option<Proof>,
    /// ChainConfig contains the configuration of the external chain
    #[prost(message, optional, tag = "3")]
    pub chain_config: ::core::option::Option<ChainConfig>,
    /// Signer represents the Desmos address associated with the
    /// profile to which link the external account
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgLinkChainAccountResponse defines the Msg/LinkAccount response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgLinkChainAccountResponse")]
pub struct MsgLinkChainAccountResponse {}
/// MsgUnlinkChainAccount represents a message to unlink an account from a
/// profile.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgUnlinkChainAccount")]
pub struct MsgUnlinkChainAccount {
    /// Owner represents the Desmos profile from which to remove the link
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// ChainName represents the name of the chain to which the link to remove is
    /// associated
    #[prost(string, tag = "2")]
    pub chain_name: ::prost::alloc::string::String,
    /// Target represents the external address to be removed
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
}
/// MsgUnlinkChainAccountResponse defines the Msg/UnlinkAccount response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgUnlinkChainAccountResponse")]
pub struct MsgUnlinkChainAccountResponse {}
/// MsgSetDefaultExternalAddress represents the message used to set a default
/// address for a specific chain
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgSetDefaultExternalAddress")]
pub struct MsgSetDefaultExternalAddress {
    /// Name of the chain for which to set the default address
    #[prost(string, tag = "1")]
    pub chain_name: ::prost::alloc::string::String,
    /// Address to be set as the default one
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// User signing the message
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSetDefaultExternalAddressResponse represents the
/// Msg/SetDefaultExternalAddress response type
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgSetDefaultExternalAddressResponse")]
pub struct MsgSetDefaultExternalAddressResponse {}
/// MsgLinkApplication defines a msg to connect a profile with a
/// centralized application account (eg. Twitter, GitHub, etc).
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgLinkApplication")]
pub struct MsgLinkApplication {
    /// The sender of the connection request
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// LinkData contains the data related to the application to which connect
    #[prost(message, optional, tag = "2")]
    pub link_data: ::core::option::Option<Data>,
    /// Hex encoded call data that will be sent to the data source in order to
    /// verify the link
    #[prost(string, tag = "3")]
    pub call_data: ::prost::alloc::string::String,
    /// The port on which the packet will be sent
    #[prost(string, tag = "4")]
    pub source_port: ::prost::alloc::string::String,
    /// The channel by which the packet will be sent
    #[prost(string, tag = "5")]
    pub source_channel: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(message, optional, tag = "6")]
    pub timeout_height: ::core::option::Option<super::super::super::ibc::core::client::v1::Height>,
    /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout_timestamp: u64,
}
/// MsgLinkApplicationResponse defines the Msg/LinkApplication
/// response type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgLinkApplicationResponse")]
pub struct MsgLinkApplicationResponse {}
/// MsgUnlinkApplication defines a msg to delete an application link from a user
/// profile
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgUnlinkApplication")]
pub struct MsgUnlinkApplication {
    /// Application represents the name of the application to unlink
    #[prost(string, tag = "1")]
    pub application: ::prost::alloc::string::String,
    /// Username represents the username inside the application to unlink
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// Signer represents the Desmos account to which the application should be
    /// unlinked
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUnlinkApplicationResponse defines the Msg/UnlinkApplication response
/// type.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgUnlinkApplicationResponse")]
pub struct MsgUnlinkApplicationResponse {}
/// LinkChainAccountPacketData defines the object that should be sent inside a
/// MsgSendPacket when wanting to link an external chain to a Desmos profile
/// using IBC
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.LinkChainAccountPacketData")]
pub struct LinkChainAccountPacketData {
    /// SourceAddress contains the details of the external chain address
    #[prost(message, optional, tag = "1")]
    pub source_address: ::core::option::Option<crate::shim::Any>,
    /// SourceProof represents the proof of ownership of the source address
    #[prost(message, optional, tag = "2")]
    pub source_proof: ::core::option::Option<Proof>,
    /// SourceChainConfig contains the details of the source chain
    #[prost(message, optional, tag = "3")]
    pub source_chain_config: ::core::option::Option<ChainConfig>,
    /// DestinationAddress represents the Desmos address of the profile that should
    /// be linked with the external account
    #[prost(string, tag = "4")]
    pub destination_address: ::prost::alloc::string::String,
    /// DestinationProof contains the proof of ownership of the DestinationAddress
    #[prost(message, optional, tag = "5")]
    pub destination_proof: ::core::option::Option<Proof>,
}
/// LinkChainAccountPacketAck defines a struct for the packet acknowledgment
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.profiles.v3.LinkChainAccountPacketAck")]
pub struct LinkChainAccountPacketAck {
    /// SourceAddress contains the external address that has been linked properly
    /// with the profile
    #[prost(string, tag = "1")]
    pub source_address: ::prost::alloc::string::String,
}
pub struct ProfilesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ProfilesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn profile(
        &self,
        user: ::prost::alloc::string::String,
    ) -> Result<QueryProfileResponse, cosmwasm_std::StdError> {
        QueryProfileRequest { user }.query(self.querier)
    }
    pub fn incoming_d_tag_transfer_requests(
        &self,
        receiver: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryIncomingDTagTransferRequestsResponse, cosmwasm_std::StdError> {
        QueryIncomingDTagTransferRequestsRequest {
            receiver,
            pagination,
        }
        .query(self.querier)
    }
    pub fn chain_links(
        &self,
        user: ::prost::alloc::string::String,
        chain_name: ::prost::alloc::string::String,
        target: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryChainLinksResponse, cosmwasm_std::StdError> {
        QueryChainLinksRequest {
            user,
            chain_name,
            target,
            pagination,
        }
        .query(self.querier)
    }
    pub fn chain_link_owners(
        &self,
        chain_name: ::prost::alloc::string::String,
        target: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryChainLinkOwnersResponse, cosmwasm_std::StdError> {
        QueryChainLinkOwnersRequest {
            chain_name,
            target,
            pagination,
        }
        .query(self.querier)
    }
    pub fn default_external_addresses(
        &self,
        owner: ::prost::alloc::string::String,
        chain_name: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryDefaultExternalAddressesResponse, cosmwasm_std::StdError> {
        QueryDefaultExternalAddressesRequest {
            owner,
            chain_name,
            pagination,
        }
        .query(self.querier)
    }
    pub fn application_links(
        &self,
        user: ::prost::alloc::string::String,
        application: ::prost::alloc::string::String,
        username: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryApplicationLinksResponse, cosmwasm_std::StdError> {
        QueryApplicationLinksRequest {
            user,
            application,
            username,
            pagination,
        }
        .query(self.querier)
    }
    pub fn application_link_by_client_id(
        &self,
        client_id: ::prost::alloc::string::String,
    ) -> Result<QueryApplicationLinkByClientIdResponse, cosmwasm_std::StdError> {
        QueryApplicationLinkByClientIdRequest { client_id }.query(self.querier)
    }
    pub fn application_link_owners(
        &self,
        application: ::prost::alloc::string::String,
        username: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryApplicationLinkOwnersResponse, cosmwasm_std::StdError> {
        QueryApplicationLinkOwnersRequest {
            application,
            username,
            pagination,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
