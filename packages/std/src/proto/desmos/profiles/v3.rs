/// QueryProfileRequest is the request type for the Query/Profile RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryProfileResponse")]
pub struct QueryProfileResponse {
    #[prost(message, optional, tag = "1")]
    pub profile: ::core::option::Option<crate::shim::Any>,
}
/// DTagTransferRequest represent a DTag transfer request between two users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.NicknameParams")]
pub struct NicknameParams {
    #[prost(bytes = "vec", tag = "1")]
    pub min_length: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub max_length: ::prost::alloc::vec::Vec<u8>,
}
/// DTagParams defines the parameters related to profile DTags
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.BioParams")]
pub struct BioParams {
    #[prost(bytes = "vec", tag = "3")]
    pub max_length: ::prost::alloc::vec::Vec<u8>,
}
/// OracleParams defines the parameters related to the oracle
/// that will be used to verify the ownership of a centralized
/// application account by a Desmos profile
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.OracleParams")]
pub struct OracleParams {
    /// ScriptID represents the ID of the oracle script to be called to verify the
    /// data
    #[prost(uint64, tag = "1")]
    pub script_id: u64,
    /// AskCount represents the number of oracles to which ask to verify the data
    #[prost(uint64, tag = "2")]
    pub ask_count: u64,
    /// MinCount represents the minimum count of oracles that should complete the
    /// verification successfully
    #[prost(uint64, tag = "3")]
    pub min_count: u64,
    /// PrepareGas represents the amount of gas to be used during the preparation
    /// stage of the oracle script
    #[prost(uint64, tag = "4")]
    pub prepare_gas: u64,
    /// ExecuteGas represents the amount of gas to be used during the execution of
    /// the oracle script
    #[prost(uint64, tag = "5")]
    pub execute_gas: u64,
    /// FeeAmount represents the amount of fees to be payed in order to execute the
    /// oracle script
    #[prost(message, repeated, tag = "6")]
    pub fee_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// AppLinksParams define the parameters related to the app links
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.AppLinksParams")]
pub struct AppLinksParams {
    /// Default validity duration before an application link expires
    #[prost(message, optional, tag = "1")]
    pub validity_duration: ::core::option::Option<crate::shim::Duration>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC endpoint
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryParamsRequest")]
#[proto_query(
    path = "/desmos.profiles.v3.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// ChainLink contains the data representing either an inter- or cross- chain
/// link
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.ChainConfig")]
pub struct ChainConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Proof contains all the data used to verify a signature when linking an
/// account to a profile
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.Base58Address")]
pub struct Base58Address {
    /// Value contains the Base58-encoded address
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// HexAddress represents an Hex-encoded address
/// NOTE: Currently it only supports keccak256-uncompressed addresses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.SingleSignature")]
pub struct SingleSignature {
    /// Type represents the type of the signature value
    #[prost(enumeration = "SignatureValueType", tag = "1")]
    pub value_type: i32,
    /// Signature is the raw signature bytes
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// CosmosMultiSignature is the signature data for a multisig public key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[derive(schemars::JsonSchema)]
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
impl SignatureValueType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignatureValueType::Unspecified => "SIGNATURE_VALUE_TYPE_UNSPECIFIED",
            SignatureValueType::Raw => "SIGNATURE_VALUE_TYPE_RAW",
            SignatureValueType::CosmosDirect => "SIGNATURE_VALUE_TYPE_COSMOS_DIRECT",
            SignatureValueType::CosmosAmino => "SIGNATURE_VALUE_TYPE_COSMOS_AMINO",
            SignatureValueType::EvmPersonalSign => "SIGNATURE_VALUE_TYPE_EVM_PERSONAL_SIGN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNATURE_VALUE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SIGNATURE_VALUE_TYPE_RAW" => Some(Self::Raw),
            "SIGNATURE_VALUE_TYPE_COSMOS_DIRECT" => Some(Self::CosmosDirect),
            "SIGNATURE_VALUE_TYPE_COSMOS_AMINO" => Some(Self::CosmosAmino),
            "SIGNATURE_VALUE_TYPE_EVM_PERSONAL_SIGN" => Some(Self::EvmPersonalSign),
            _ => None,
        }
    }
}
/// QueryChainLinksRequest represents the request that should be used in order
/// to retrieve the link associated with the provided user, for the given chain
/// and having the given target address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    /// ChainLinkOwnerDetails contains the details of a single chain link owner
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.ApplicationLink")]
pub struct ApplicationLink {
    ///   User to which the link is associated
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// Data contains the details of this specific link
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    /// State of the link
    #[prost(enumeration = "ApplicationLinkState", tag = "3")]
    pub state: i32,
    /// OracleRequest represents the request that has been made to the oracle
    #[prost(message, optional, tag = "4")]
    pub oracle_request: ::core::option::Option<OracleRequest>,
    /// Data coming from the result of the verification.
    /// Only available when the state is STATE_SUCCESS
    #[prost(message, optional, tag = "5")]
    pub result: ::core::option::Option<Result>,
    /// CreationTime represents the time in which the link was created
    #[prost(message, optional, tag = "6")]
    pub creation_time: ::core::option::Option<crate::shim::Timestamp>,
    /// ExpirationTime represents the time in which the link will expire
    #[prost(message, optional, tag = "7")]
    pub expiration_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// Data contains the data associated to a specific user of a
/// generic centralized application
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.OracleRequest")]
pub struct OracleRequest {
    /// ID is the ID of the request
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// OracleScriptID is ID of an oracle script
    #[prost(uint64, tag = "2")]
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
    /// CallData contains the data sent to a single oracle request in order to
    /// verify the ownership of a centralized application by a Desmos profile
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.Result")]
pub struct Result {
    /// sum is the oneof that specifies whether this represents a success or
    /// failure result
    #[prost(oneof = "result::Sum", tags = "1, 2")]
    pub sum: ::core::option::Option<result::Sum>,
}
/// Nested message and enum types in `Result`.
pub mod result {
    /// Success is the result of an application link that has been successfully
    /// verified
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
    #[proto_message(type_url = "/desmos.profiles.v3.Result.Failed")]
    pub struct Failed {
        /// Error that is associated with the failure
        #[prost(string, tag = "1")]
        pub error: ::prost::alloc::string::String,
    }
    /// sum is the oneof that specifies whether this represents a success or
    /// failure result
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof, schemars::JsonSchema)]
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
#[derive(schemars::JsonSchema)]
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
impl ApplicationLinkState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApplicationLinkState::InitializedUnspecified => {
                "APPLICATION_LINK_STATE_INITIALIZED_UNSPECIFIED"
            }
            ApplicationLinkState::VerificationStarted => {
                "APPLICATION_LINK_STATE_VERIFICATION_STARTED"
            }
            ApplicationLinkState::VerificationError => "APPLICATION_LINK_STATE_VERIFICATION_ERROR",
            ApplicationLinkState::VerificationSuccess => {
                "APPLICATION_LINK_STATE_VERIFICATION_SUCCESS"
            }
            ApplicationLinkState::TimedOut => "APPLICATION_LINK_STATE_TIMED_OUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "APPLICATION_LINK_STATE_INITIALIZED_UNSPECIFIED" => Some(Self::InitializedUnspecified),
            "APPLICATION_LINK_STATE_VERIFICATION_STARTED" => Some(Self::VerificationStarted),
            "APPLICATION_LINK_STATE_VERIFICATION_ERROR" => Some(Self::VerificationError),
            "APPLICATION_LINK_STATE_VERIFICATION_SUCCESS" => Some(Self::VerificationSuccess),
            "APPLICATION_LINK_STATE_TIMED_OUT" => Some(Self::TimedOut),
            _ => None,
        }
    }
}
/// QueryUserApplicationLinkRequest represents the request used when querying an
/// application link using an application name and username for a given user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.QueryApplicationLinkByClientIDResponse")]
pub struct QueryApplicationLinkByClientIdResponse {
    #[prost(message, optional, tag = "1")]
    pub link: ::core::option::Option<ApplicationLink>,
}
/// QueryApplicationLinkOwnersRequest contains the data of the request that can
/// be used to get application link owners
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    /// ApplicationLinkOwnerDetails contains the details of a single application
    /// link owner
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgRequestDTagTransferResponse")]
pub struct MsgRequestDTagTransferResponse {}
/// MsgCancelDTagTransferRequest represents the message used to cancel a DTag
/// transfer request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgCancelDTagTransferRequestResponse")]
pub struct MsgCancelDTagTransferRequestResponse {}
/// MsgAcceptDTagTransferRequest represents the message used to accept a DTag
/// transfer request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgAcceptDTagTransferRequestResponse")]
pub struct MsgAcceptDTagTransferRequestResponse {}
/// MsgRefuseDTagTransferRequest represents the message used to refuse a DTag
/// transfer request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgRefuseDTagTransferRequestResponse")]
pub struct MsgRefuseDTagTransferRequestResponse {}
/// MsgSaveProfile represents a message to save a profile.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgSaveProfileResponse")]
pub struct MsgSaveProfileResponse {}
/// MsgDeleteProfile represents the message used to delete an existing profile.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgDeleteProfile")]
pub struct MsgDeleteProfile {
    /// Address associated to the profile to be deleted
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeleteProfileResponse defines the Msg/DeleteProfile response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgDeleteProfileResponse")]
pub struct MsgDeleteProfileResponse {}
/// MsgLinkChainAccount represents a message to link an account to a profile.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgLinkChainAccountResponse")]
pub struct MsgLinkChainAccountResponse {}
/// MsgUnlinkChainAccount represents a message to unlink an account from a
/// profile.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgUnlinkChainAccountResponse")]
pub struct MsgUnlinkChainAccountResponse {}
/// MsgSetDefaultExternalAddress represents the message used to set a default
/// address for a specific chain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgSetDefaultExternalAddressResponse")]
pub struct MsgSetDefaultExternalAddressResponse {}
/// MsgLinkApplication defines a msg to connect a profile with a
/// centralized application account (eg. Twitter, GitHub, etc).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    pub timeout_timestamp: u64,
}
/// MsgLinkApplicationResponse defines the Msg/LinkApplication
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgLinkApplicationResponse")]
pub struct MsgLinkApplicationResponse {}
/// MsgUnlinkApplication defines a msg to delete an application link from a user
/// profile
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v3.MsgUnlinkApplicationResponse")]
pub struct MsgUnlinkApplicationResponse {}
/// LinkChainAccountPacketData defines the object that should be sent inside a
/// MsgSendPacket when wanting to link an external chain to a Desmos profile
/// using IBC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
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
    ) -> std::result::Result<QueryProfileResponse, cosmwasm_std::StdError> {
        QueryProfileRequest { user }.query(self.querier)
    }
    pub fn incoming_d_tag_transfer_requests(
        &self,
        receiver: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryIncomingDTagTransferRequestsResponse, cosmwasm_std::StdError>
    {
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
    ) -> std::result::Result<QueryChainLinksResponse, cosmwasm_std::StdError> {
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
    ) -> std::result::Result<QueryChainLinkOwnersResponse, cosmwasm_std::StdError> {
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
    ) -> std::result::Result<QueryDefaultExternalAddressesResponse, cosmwasm_std::StdError> {
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
    ) -> std::result::Result<QueryApplicationLinksResponse, cosmwasm_std::StdError> {
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
    ) -> std::result::Result<QueryApplicationLinkByClientIdResponse, cosmwasm_std::StdError> {
        QueryApplicationLinkByClientIdRequest { client_id }.query(self.querier)
    }
    pub fn application_link_owners(
        &self,
        application: ::prost::alloc::string::String,
        username: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryApplicationLinkOwnersResponse, cosmwasm_std::StdError> {
        QueryApplicationLinkOwnersRequest {
            application,
            username,
            pagination,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
impl serde::Serialize for AppLinksParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.validity_duration.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.AppLinksParams", len)?;
        if let Some(v) = self.validity_duration.as_ref() {
            struct_ser.serialize_field("validityDuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppLinksParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validity_duration", "validityDuration"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidityDuration,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "validityDuration" | "validity_duration" => {
                                Ok(GeneratedField::ValidityDuration)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppLinksParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.AppLinksParams")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<AppLinksParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validity_duration__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidityDuration => {
                            if validity_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validityDuration"));
                            }
                            validity_duration__ = map.next_value()?;
                        }
                    }
                }
                Ok(AppLinksParams {
                    validity_duration: validity_duration__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.AppLinksParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ApplicationLink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.oracle_request.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        if self.creation_time.is_some() {
            len += 1;
        }
        if self.expiration_time.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.ApplicationLink", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if self.state != 0 {
            let v = ApplicationLinkState::from_i32(self.state).ok_or_else(|| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.state))
            })?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.oracle_request.as_ref() {
            struct_ser.serialize_field("oracleRequest", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        if let Some(v) = self.creation_time.as_ref() {
            struct_ser.serialize_field("creationTime", v)?;
        }
        if let Some(v) = self.expiration_time.as_ref() {
            struct_ser.serialize_field("expirationTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationLink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "data",
            "state",
            "oracle_request",
            "oracleRequest",
            "result",
            "creation_time",
            "creationTime",
            "expiration_time",
            "expirationTime",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Data,
            State,
            OracleRequest,
            Result,
            CreationTime,
            ExpirationTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "data" => Ok(GeneratedField::Data),
                            "state" => Ok(GeneratedField::State),
                            "oracleRequest" | "oracle_request" => Ok(GeneratedField::OracleRequest),
                            "result" => Ok(GeneratedField::Result),
                            "creationTime" | "creation_time" => Ok(GeneratedField::CreationTime),
                            "expirationTime" | "expiration_time" => {
                                Ok(GeneratedField::ExpirationTime)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.ApplicationLink")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApplicationLink, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut data__ = None;
                let mut state__ = None;
                let mut oracle_request__ = None;
                let mut result__ = None;
                let mut creation_time__ = None;
                let mut expiration_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map.next_value::<ApplicationLinkState>()? as i32);
                        }
                        GeneratedField::OracleRequest => {
                            if oracle_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleRequest"));
                            }
                            oracle_request__ = map.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                        GeneratedField::CreationTime => {
                            if creation_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTime"));
                            }
                            creation_time__ = map.next_value()?;
                        }
                        GeneratedField::ExpirationTime => {
                            if expiration_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationTime"));
                            }
                            expiration_time__ = map.next_value()?;
                        }
                    }
                }
                Ok(ApplicationLink {
                    user: user__.unwrap_or_default(),
                    data: data__,
                    state: state__.unwrap_or_default(),
                    oracle_request: oracle_request__,
                    result: result__,
                    creation_time: creation_time__,
                    expiration_time: expiration_time__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.ApplicationLink",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ApplicationLinkState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::InitializedUnspecified => "APPLICATION_LINK_STATE_INITIALIZED_UNSPECIFIED",
            Self::VerificationStarted => "APPLICATION_LINK_STATE_VERIFICATION_STARTED",
            Self::VerificationError => "APPLICATION_LINK_STATE_VERIFICATION_ERROR",
            Self::VerificationSuccess => "APPLICATION_LINK_STATE_VERIFICATION_SUCCESS",
            Self::TimedOut => "APPLICATION_LINK_STATE_TIMED_OUT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationLinkState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "APPLICATION_LINK_STATE_INITIALIZED_UNSPECIFIED",
            "APPLICATION_LINK_STATE_VERIFICATION_STARTED",
            "APPLICATION_LINK_STATE_VERIFICATION_ERROR",
            "APPLICATION_LINK_STATE_VERIFICATION_SUCCESS",
            "APPLICATION_LINK_STATE_TIMED_OUT",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationLinkState;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(ApplicationLinkState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(ApplicationLinkState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "APPLICATION_LINK_STATE_INITIALIZED_UNSPECIFIED" => {
                        Ok(ApplicationLinkState::InitializedUnspecified)
                    }
                    "APPLICATION_LINK_STATE_VERIFICATION_STARTED" => {
                        Ok(ApplicationLinkState::VerificationStarted)
                    }
                    "APPLICATION_LINK_STATE_VERIFICATION_ERROR" => {
                        Ok(ApplicationLinkState::VerificationError)
                    }
                    "APPLICATION_LINK_STATE_VERIFICATION_SUCCESS" => {
                        Ok(ApplicationLinkState::VerificationSuccess)
                    }
                    "APPLICATION_LINK_STATE_TIMED_OUT" => Ok(ApplicationLinkState::TimedOut),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Base58Address {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.Base58Address", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Base58Address {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Base58Address;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Base58Address")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Base58Address, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Base58Address {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.Base58Address",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Bech32Address {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.Bech32Address", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bech32Address {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value", "prefix"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Prefix,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "prefix" => Ok(GeneratedField::Prefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Bech32Address;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Bech32Address")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Bech32Address, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Bech32Address {
                    value: value__.unwrap_or_default(),
                    prefix: prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.Bech32Address",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BioParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.max_length.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.BioParams", len)?;
        if !self.max_length.is_empty() {
            struct_ser.serialize_field(
                "maxLength",
                pbjson::private::base64::encode(&self.max_length).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BioParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["max_length", "maxLength"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxLength,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxLength" | "max_length" => Ok(GeneratedField::MaxLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BioParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.BioParams")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<BioParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut max_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxLength => {
                            if max_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLength"));
                            }
                            max_length__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(BioParams {
                    max_length: max_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.BioParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChainConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.ChainConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainConfig;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.ChainConfig")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ChainConfig {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.ChainConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChainLink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.proof.is_some() {
            len += 1;
        }
        if self.chain_config.is_some() {
            len += 1;
        }
        if self.creation_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.ChainLink", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.proof.as_ref() {
            struct_ser.serialize_field("proof", v)?;
        }
        if let Some(v) = self.chain_config.as_ref() {
            struct_ser.serialize_field("chainConfig", v)?;
        }
        if let Some(v) = self.creation_time.as_ref() {
            struct_ser.serialize_field("creationTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainLink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "address",
            "proof",
            "chain_config",
            "chainConfig",
            "creation_time",
            "creationTime",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Address,
            Proof,
            ChainConfig,
            CreationTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "address" => Ok(GeneratedField::Address),
                            "proof" => Ok(GeneratedField::Proof),
                            "chainConfig" | "chain_config" => Ok(GeneratedField::ChainConfig),
                            "creationTime" | "creation_time" => Ok(GeneratedField::CreationTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.ChainLink")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainLink, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut address__ = None;
                let mut proof__ = None;
                let mut chain_config__ = None;
                let mut creation_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = map.next_value()?;
                        }
                        GeneratedField::ChainConfig => {
                            if chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainConfig"));
                            }
                            chain_config__ = map.next_value()?;
                        }
                        GeneratedField::CreationTime => {
                            if creation_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTime"));
                            }
                            creation_time__ = map.next_value()?;
                        }
                    }
                }
                Ok(ChainLink {
                    user: user__.unwrap_or_default(),
                    address: address__,
                    proof: proof__,
                    chain_config: chain_config__,
                    creation_time: creation_time__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.ChainLink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CosmosMultiSignature {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bit_array.is_some() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.CosmosMultiSignature", len)?;
        if let Some(v) = self.bit_array.as_ref() {
            struct_ser.serialize_field("bitArray", v)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CosmosMultiSignature {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["bit_array", "bitArray", "signatures"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BitArray,
            Signatures,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bitArray" | "bit_array" => Ok(GeneratedField::BitArray),
                            "signatures" => Ok(GeneratedField::Signatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CosmosMultiSignature;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.CosmosMultiSignature")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<CosmosMultiSignature, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bit_array__ = None;
                let mut signatures__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BitArray => {
                            if bit_array__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitArray"));
                            }
                            bit_array__ = map.next_value()?;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CosmosMultiSignature {
                    bit_array: bit_array__,
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.CosmosMultiSignature",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DTagParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reg_ex.is_empty() {
            len += 1;
        }
        if !self.min_length.is_empty() {
            len += 1;
        }
        if !self.max_length.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.DTagParams", len)?;
        if !self.reg_ex.is_empty() {
            struct_ser.serialize_field("regEx", &self.reg_ex)?;
        }
        if !self.min_length.is_empty() {
            struct_ser.serialize_field(
                "minLength",
                pbjson::private::base64::encode(&self.min_length).as_str(),
            )?;
        }
        if !self.max_length.is_empty() {
            struct_ser.serialize_field(
                "maxLength",
                pbjson::private::base64::encode(&self.max_length).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DTagParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reg_ex",
            "regEx",
            "min_length",
            "minLength",
            "max_length",
            "maxLength",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegEx,
            MinLength,
            MaxLength,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "regEx" | "reg_ex" => Ok(GeneratedField::RegEx),
                            "minLength" | "min_length" => Ok(GeneratedField::MinLength),
                            "maxLength" | "max_length" => Ok(GeneratedField::MaxLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DTagParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.DTagParams")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<DTagParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut reg_ex__ = None;
                let mut min_length__ = None;
                let mut max_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegEx => {
                            if reg_ex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regEx"));
                            }
                            reg_ex__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinLength => {
                            if min_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLength"));
                            }
                            min_length__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxLength => {
                            if max_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLength"));
                            }
                            max_length__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(DTagParams {
                    reg_ex: reg_ex__.unwrap_or_default(),
                    min_length: min_length__.unwrap_or_default(),
                    max_length: max_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.DTagParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DTagTransferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dtag_to_trade.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.receiver.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.DTagTransferRequest", len)?;
        if !self.dtag_to_trade.is_empty() {
            struct_ser.serialize_field("dtagToTrade", &self.dtag_to_trade)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.receiver.is_empty() {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DTagTransferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["dtag_to_trade", "dtagToTrade", "sender", "receiver"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DtagToTrade,
            Sender,
            Receiver,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dtagToTrade" | "dtag_to_trade" => Ok(GeneratedField::DtagToTrade),
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DTagTransferRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.DTagTransferRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<DTagTransferRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut dtag_to_trade__ = None;
                let mut sender__ = None;
                let mut receiver__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DtagToTrade => {
                            if dtag_to_trade__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dtagToTrade"));
                            }
                            dtag_to_trade__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DTagTransferRequest {
                    dtag_to_trade: dtag_to_trade__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.DTagTransferRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Data {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.application.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.Data", len)?;
        if !self.application.is_empty() {
            struct_ser.serialize_field("application", &self.application)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Data {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["application", "username"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Application,
            Username,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "application" => Ok(GeneratedField::Application),
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Data;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Data")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Data, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut application__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Application => {
                            if application__.is_some() {
                                return Err(serde::de::Error::duplicate_field("application"));
                            }
                            application__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Data {
                    application: application__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.Data", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HexAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.HexAddress", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HexAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value", "prefix"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Prefix,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "prefix" => Ok(GeneratedField::Prefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HexAddress;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.HexAddress")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<HexAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HexAddress {
                    value: value__.unwrap_or_default(),
                    prefix: prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.HexAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinkChainAccountPacketAck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_address.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.LinkChainAccountPacketAck", len)?;
        if !self.source_address.is_empty() {
            struct_ser.serialize_field("sourceAddress", &self.source_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkChainAccountPacketAck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["source_address", "sourceAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sourceAddress" | "source_address" => Ok(GeneratedField::SourceAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinkChainAccountPacketAck;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.LinkChainAccountPacketAck")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<LinkChainAccountPacketAck, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceAddress => {
                            if source_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceAddress"));
                            }
                            source_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LinkChainAccountPacketAck {
                    source_address: source_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.LinkChainAccountPacketAck",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for LinkChainAccountPacketData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_address.is_some() {
            len += 1;
        }
        if self.source_proof.is_some() {
            len += 1;
        }
        if self.source_chain_config.is_some() {
            len += 1;
        }
        if !self.destination_address.is_empty() {
            len += 1;
        }
        if self.destination_proof.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.LinkChainAccountPacketData", len)?;
        if let Some(v) = self.source_address.as_ref() {
            struct_ser.serialize_field("sourceAddress", v)?;
        }
        if let Some(v) = self.source_proof.as_ref() {
            struct_ser.serialize_field("sourceProof", v)?;
        }
        if let Some(v) = self.source_chain_config.as_ref() {
            struct_ser.serialize_field("sourceChainConfig", v)?;
        }
        if !self.destination_address.is_empty() {
            struct_ser.serialize_field("destinationAddress", &self.destination_address)?;
        }
        if let Some(v) = self.destination_proof.as_ref() {
            struct_ser.serialize_field("destinationProof", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkChainAccountPacketData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_address",
            "sourceAddress",
            "source_proof",
            "sourceProof",
            "source_chain_config",
            "sourceChainConfig",
            "destination_address",
            "destinationAddress",
            "destination_proof",
            "destinationProof",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceAddress,
            SourceProof,
            SourceChainConfig,
            DestinationAddress,
            DestinationProof,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sourceAddress" | "source_address" => Ok(GeneratedField::SourceAddress),
                            "sourceProof" | "source_proof" => Ok(GeneratedField::SourceProof),
                            "sourceChainConfig" | "source_chain_config" => {
                                Ok(GeneratedField::SourceChainConfig)
                            }
                            "destinationAddress" | "destination_address" => {
                                Ok(GeneratedField::DestinationAddress)
                            }
                            "destinationProof" | "destination_proof" => {
                                Ok(GeneratedField::DestinationProof)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinkChainAccountPacketData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.LinkChainAccountPacketData")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<LinkChainAccountPacketData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source_address__ = None;
                let mut source_proof__ = None;
                let mut source_chain_config__ = None;
                let mut destination_address__ = None;
                let mut destination_proof__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceAddress => {
                            if source_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceAddress"));
                            }
                            source_address__ = map.next_value()?;
                        }
                        GeneratedField::SourceProof => {
                            if source_proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceProof"));
                            }
                            source_proof__ = map.next_value()?;
                        }
                        GeneratedField::SourceChainConfig => {
                            if source_chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceChainConfig"));
                            }
                            source_chain_config__ = map.next_value()?;
                        }
                        GeneratedField::DestinationAddress => {
                            if destination_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "destinationAddress",
                                ));
                            }
                            destination_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::DestinationProof => {
                            if destination_proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationProof"));
                            }
                            destination_proof__ = map.next_value()?;
                        }
                    }
                }
                Ok(LinkChainAccountPacketData {
                    source_address: source_address__,
                    source_proof: source_proof__,
                    source_chain_config: source_chain_config__,
                    destination_address: destination_address__.unwrap_or_default(),
                    destination_proof: destination_proof__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.LinkChainAccountPacketData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAcceptDTagTransferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.new_dtag.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.receiver.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgAcceptDTagTransferRequest", len)?;
        if !self.new_dtag.is_empty() {
            struct_ser.serialize_field("newDtag", &self.new_dtag)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.receiver.is_empty() {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAcceptDTagTransferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["new_dtag", "newDtag", "sender", "receiver"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NewDtag,
            Sender,
            Receiver,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "newDtag" | "new_dtag" => Ok(GeneratedField::NewDtag),
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAcceptDTagTransferRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgAcceptDTagTransferRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAcceptDTagTransferRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut new_dtag__ = None;
                let mut sender__ = None;
                let mut receiver__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NewDtag => {
                            if new_dtag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newDtag"));
                            }
                            new_dtag__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgAcceptDTagTransferRequest {
                    new_dtag: new_dtag__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgAcceptDTagTransferRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgAcceptDTagTransferRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.MsgAcceptDTagTransferRequestResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAcceptDTagTransferRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAcceptDTagTransferRequestResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.MsgAcceptDTagTransferRequestResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAcceptDTagTransferRequestResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAcceptDTagTransferRequestResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgAcceptDTagTransferRequestResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgCancelDTagTransferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.receiver.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgCancelDTagTransferRequest", len)?;
        if !self.receiver.is_empty() {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelDTagTransferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["receiver", "sender"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Receiver,
            Sender,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "receiver" => Ok(GeneratedField::Receiver),
                            "sender" => Ok(GeneratedField::Sender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancelDTagTransferRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgCancelDTagTransferRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCancelDTagTransferRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut receiver__ = None;
                let mut sender__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCancelDTagTransferRequest {
                    receiver: receiver__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgCancelDTagTransferRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgCancelDTagTransferRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.MsgCancelDTagTransferRequestResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelDTagTransferRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancelDTagTransferRequestResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.MsgCancelDTagTransferRequestResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCancelDTagTransferRequestResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCancelDTagTransferRequestResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgCancelDTagTransferRequestResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeleteProfile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.creator.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgDeleteProfile", len)?;
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeleteProfile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["creator"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Creator,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "creator" => Ok(GeneratedField::Creator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDeleteProfile;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgDeleteProfile")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgDeleteProfile, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut creator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgDeleteProfile {
                    creator: creator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgDeleteProfile",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgDeleteProfileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgDeleteProfileResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDeleteProfileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDeleteProfileResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgDeleteProfileResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDeleteProfileResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDeleteProfileResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgDeleteProfileResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgLinkApplication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if self.link_data.is_some() {
            len += 1;
        }
        if !self.call_data.is_empty() {
            len += 1;
        }
        if !self.source_port.is_empty() {
            len += 1;
        }
        if !self.source_channel.is_empty() {
            len += 1;
        }
        if self.timeout_height.is_some() {
            len += 1;
        }
        if self.timeout_timestamp != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgLinkApplication", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if let Some(v) = self.link_data.as_ref() {
            struct_ser.serialize_field("linkData", v)?;
        }
        if !self.call_data.is_empty() {
            struct_ser.serialize_field("callData", &self.call_data)?;
        }
        if !self.source_port.is_empty() {
            struct_ser.serialize_field("sourcePort", &self.source_port)?;
        }
        if !self.source_channel.is_empty() {
            struct_ser.serialize_field("sourceChannel", &self.source_channel)?;
        }
        if let Some(v) = self.timeout_height.as_ref() {
            struct_ser.serialize_field("timeoutHeight", v)?;
        }
        if self.timeout_timestamp != 0 {
            struct_ser.serialize_field(
                "timeoutTimestamp",
                ToString::to_string(&self.timeout_timestamp).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLinkApplication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "link_data",
            "linkData",
            "call_data",
            "callData",
            "source_port",
            "sourcePort",
            "source_channel",
            "sourceChannel",
            "timeout_height",
            "timeoutHeight",
            "timeout_timestamp",
            "timeoutTimestamp",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            LinkData,
            CallData,
            SourcePort,
            SourceChannel,
            TimeoutHeight,
            TimeoutTimestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "linkData" | "link_data" => Ok(GeneratedField::LinkData),
                            "callData" | "call_data" => Ok(GeneratedField::CallData),
                            "sourcePort" | "source_port" => Ok(GeneratedField::SourcePort),
                            "sourceChannel" | "source_channel" => Ok(GeneratedField::SourceChannel),
                            "timeoutHeight" | "timeout_height" => Ok(GeneratedField::TimeoutHeight),
                            "timeoutTimestamp" | "timeout_timestamp" => {
                                Ok(GeneratedField::TimeoutTimestamp)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLinkApplication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgLinkApplication")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgLinkApplication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut link_data__ = None;
                let mut call_data__ = None;
                let mut source_port__ = None;
                let mut source_channel__ = None;
                let mut timeout_height__ = None;
                let mut timeout_timestamp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::LinkData => {
                            if link_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkData"));
                            }
                            link_data__ = map.next_value()?;
                        }
                        GeneratedField::CallData => {
                            if call_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callData"));
                            }
                            call_data__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourcePort => {
                            if source_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePort"));
                            }
                            source_port__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceChannel => {
                            if source_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceChannel"));
                            }
                            source_channel__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimeoutHeight => {
                            if timeout_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutHeight"));
                            }
                            timeout_height__ = map.next_value()?;
                        }
                        GeneratedField::TimeoutTimestamp => {
                            if timeout_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutTimestamp"));
                            }
                            timeout_timestamp__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgLinkApplication {
                    sender: sender__.unwrap_or_default(),
                    link_data: link_data__,
                    call_data: call_data__.unwrap_or_default(),
                    source_port: source_port__.unwrap_or_default(),
                    source_channel: source_channel__.unwrap_or_default(),
                    timeout_height: timeout_height__,
                    timeout_timestamp: timeout_timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgLinkApplication",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgLinkApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgLinkApplicationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLinkApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLinkApplicationResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgLinkApplicationResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgLinkApplicationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgLinkApplicationResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgLinkApplicationResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgLinkChainAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_address.is_some() {
            len += 1;
        }
        if self.proof.is_some() {
            len += 1;
        }
        if self.chain_config.is_some() {
            len += 1;
        }
        if !self.signer.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgLinkChainAccount", len)?;
        if let Some(v) = self.chain_address.as_ref() {
            struct_ser.serialize_field("chainAddress", v)?;
        }
        if let Some(v) = self.proof.as_ref() {
            struct_ser.serialize_field("proof", v)?;
        }
        if let Some(v) = self.chain_config.as_ref() {
            struct_ser.serialize_field("chainConfig", v)?;
        }
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLinkChainAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_address",
            "chainAddress",
            "proof",
            "chain_config",
            "chainConfig",
            "signer",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainAddress,
            Proof,
            ChainConfig,
            Signer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainAddress" | "chain_address" => Ok(GeneratedField::ChainAddress),
                            "proof" => Ok(GeneratedField::Proof),
                            "chainConfig" | "chain_config" => Ok(GeneratedField::ChainConfig),
                            "signer" => Ok(GeneratedField::Signer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLinkChainAccount;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgLinkChainAccount")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgLinkChainAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_address__ = None;
                let mut proof__ = None;
                let mut chain_config__ = None;
                let mut signer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainAddress => {
                            if chain_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainAddress"));
                            }
                            chain_address__ = map.next_value()?;
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = map.next_value()?;
                        }
                        GeneratedField::ChainConfig => {
                            if chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainConfig"));
                            }
                            chain_config__ = map.next_value()?;
                        }
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgLinkChainAccount {
                    chain_address: chain_address__,
                    proof: proof__,
                    chain_config: chain_config__,
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgLinkChainAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgLinkChainAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgLinkChainAccountResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLinkChainAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLinkChainAccountResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgLinkChainAccountResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgLinkChainAccountResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgLinkChainAccountResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgLinkChainAccountResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRefuseDTagTransferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.receiver.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgRefuseDTagTransferRequest", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.receiver.is_empty() {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRefuseDTagTransferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "receiver"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Receiver,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "receiver" => Ok(GeneratedField::Receiver),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRefuseDTagTransferRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgRefuseDTagTransferRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRefuseDTagTransferRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut receiver__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgRefuseDTagTransferRequest {
                    sender: sender__.unwrap_or_default(),
                    receiver: receiver__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgRefuseDTagTransferRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRefuseDTagTransferRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.MsgRefuseDTagTransferRequestResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRefuseDTagTransferRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRefuseDTagTransferRequestResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.MsgRefuseDTagTransferRequestResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRefuseDTagTransferRequestResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRefuseDTagTransferRequestResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgRefuseDTagTransferRequestResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRequestDTagTransfer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.receiver.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgRequestDTagTransfer", len)?;
        if !self.receiver.is_empty() {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRequestDTagTransfer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["receiver", "sender"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Receiver,
            Sender,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "receiver" => Ok(GeneratedField::Receiver),
                            "sender" => Ok(GeneratedField::Sender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRequestDTagTransfer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgRequestDTagTransfer")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRequestDTagTransfer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut receiver__ = None;
                let mut sender__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgRequestDTagTransfer {
                    receiver: receiver__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgRequestDTagTransfer",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgRequestDTagTransferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("desmos.profiles.v3.MsgRequestDTagTransferResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRequestDTagTransferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRequestDTagTransferResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgRequestDTagTransferResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRequestDTagTransferResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRequestDTagTransferResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgRequestDTagTransferResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSaveProfile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dtag.is_empty() {
            len += 1;
        }
        if !self.nickname.is_empty() {
            len += 1;
        }
        if !self.bio.is_empty() {
            len += 1;
        }
        if !self.profile_picture.is_empty() {
            len += 1;
        }
        if !self.cover_picture.is_empty() {
            len += 1;
        }
        if !self.creator.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgSaveProfile", len)?;
        if !self.dtag.is_empty() {
            struct_ser.serialize_field("dtag", &self.dtag)?;
        }
        if !self.nickname.is_empty() {
            struct_ser.serialize_field("nickname", &self.nickname)?;
        }
        if !self.bio.is_empty() {
            struct_ser.serialize_field("bio", &self.bio)?;
        }
        if !self.profile_picture.is_empty() {
            struct_ser.serialize_field("profilePicture", &self.profile_picture)?;
        }
        if !self.cover_picture.is_empty() {
            struct_ser.serialize_field("coverPicture", &self.cover_picture)?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSaveProfile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dtag",
            "nickname",
            "bio",
            "profile_picture",
            "profilePicture",
            "cover_picture",
            "coverPicture",
            "creator",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dtag,
            Nickname,
            Bio,
            ProfilePicture,
            CoverPicture,
            Creator,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dtag" => Ok(GeneratedField::Dtag),
                            "nickname" => Ok(GeneratedField::Nickname),
                            "bio" => Ok(GeneratedField::Bio),
                            "profilePicture" | "profile_picture" => {
                                Ok(GeneratedField::ProfilePicture)
                            }
                            "coverPicture" | "cover_picture" => Ok(GeneratedField::CoverPicture),
                            "creator" => Ok(GeneratedField::Creator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSaveProfile;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgSaveProfile")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSaveProfile, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut dtag__ = None;
                let mut nickname__ = None;
                let mut bio__ = None;
                let mut profile_picture__ = None;
                let mut cover_picture__ = None;
                let mut creator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Dtag => {
                            if dtag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dtag"));
                            }
                            dtag__ = Some(map.next_value()?);
                        }
                        GeneratedField::Nickname => {
                            if nickname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickname"));
                            }
                            nickname__ = Some(map.next_value()?);
                        }
                        GeneratedField::Bio => {
                            if bio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bio"));
                            }
                            bio__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProfilePicture => {
                            if profile_picture__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profilePicture"));
                            }
                            profile_picture__ = Some(map.next_value()?);
                        }
                        GeneratedField::CoverPicture => {
                            if cover_picture__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverPicture"));
                            }
                            cover_picture__ = Some(map.next_value()?);
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgSaveProfile {
                    dtag: dtag__.unwrap_or_default(),
                    nickname: nickname__.unwrap_or_default(),
                    bio: bio__.unwrap_or_default(),
                    profile_picture: profile_picture__.unwrap_or_default(),
                    cover_picture: cover_picture__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgSaveProfile",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSaveProfileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgSaveProfileResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSaveProfileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSaveProfileResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgSaveProfileResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSaveProfileResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSaveProfileResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgSaveProfileResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetDefaultExternalAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_name.is_empty() {
            len += 1;
        }
        if !self.target.is_empty() {
            len += 1;
        }
        if !self.signer.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgSetDefaultExternalAddress", len)?;
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetDefaultExternalAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_name", "chainName", "target", "signer"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainName,
            Target,
            Signer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            "target" => Ok(GeneratedField::Target),
                            "signer" => Ok(GeneratedField::Signer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetDefaultExternalAddress;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgSetDefaultExternalAddress")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSetDefaultExternalAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_name__ = None;
                let mut target__ = None;
                let mut signer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map.next_value()?);
                        }
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgSetDefaultExternalAddress {
                    chain_name: chain_name__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgSetDefaultExternalAddress",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgSetDefaultExternalAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.MsgSetDefaultExternalAddressResponse",
            len,
        )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetDefaultExternalAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetDefaultExternalAddressResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.MsgSetDefaultExternalAddressResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSetDefaultExternalAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetDefaultExternalAddressResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgSetDefaultExternalAddressResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlinkApplication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.application.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.signer.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgUnlinkApplication", len)?;
        if !self.application.is_empty() {
            struct_ser.serialize_field("application", &self.application)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.signer.is_empty() {
            struct_ser.serialize_field("signer", &self.signer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlinkApplication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["application", "username", "signer"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Application,
            Username,
            Signer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "application" => Ok(GeneratedField::Application),
                            "username" => Ok(GeneratedField::Username),
                            "signer" => Ok(GeneratedField::Signer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnlinkApplication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgUnlinkApplication")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUnlinkApplication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut application__ = None;
                let mut username__ = None;
                let mut signer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Application => {
                            if application__.is_some() {
                                return Err(serde::de::Error::duplicate_field("application"));
                            }
                            application__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::Signer => {
                            if signer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            signer__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUnlinkApplication {
                    application: application__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    signer: signer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgUnlinkApplication",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlinkApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgUnlinkApplicationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlinkApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnlinkApplicationResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgUnlinkApplicationResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnlinkApplicationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnlinkApplicationResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgUnlinkApplicationResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlinkChainAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.chain_name.is_empty() {
            len += 1;
        }
        if !self.target.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgUnlinkChainAccount", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlinkChainAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["owner", "chain_name", "chainName", "target"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            ChainName,
            Target,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "owner" => Ok(GeneratedField::Owner),
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnlinkChainAccount;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgUnlinkChainAccount")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnlinkChainAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut chain_name__ = None;
                let mut target__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUnlinkChainAccount {
                    owner: owner__.unwrap_or_default(),
                    chain_name: chain_name__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgUnlinkChainAccount",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for MsgUnlinkChainAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.MsgUnlinkChainAccountResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnlinkChainAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUnlinkChainAccountResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.MsgUnlinkChainAccountResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnlinkChainAccountResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnlinkChainAccountResponse {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.MsgUnlinkChainAccountResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for NicknameParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.min_length.is_empty() {
            len += 1;
        }
        if !self.max_length.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.NicknameParams", len)?;
        if !self.min_length.is_empty() {
            struct_ser.serialize_field(
                "minLength",
                pbjson::private::base64::encode(&self.min_length).as_str(),
            )?;
        }
        if !self.max_length.is_empty() {
            struct_ser.serialize_field(
                "maxLength",
                pbjson::private::base64::encode(&self.max_length).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NicknameParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["min_length", "minLength", "max_length", "maxLength"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinLength,
            MaxLength,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minLength" | "min_length" => Ok(GeneratedField::MinLength),
                            "maxLength" | "max_length" => Ok(GeneratedField::MaxLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NicknameParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.NicknameParams")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<NicknameParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut min_length__ = None;
                let mut max_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MinLength => {
                            if min_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLength"));
                            }
                            min_length__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxLength => {
                            if max_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLength"));
                            }
                            max_length__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(NicknameParams {
                    min_length: min_length__.unwrap_or_default(),
                    max_length: max_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.NicknameParams",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for OracleParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.script_id != 0 {
            len += 1;
        }
        if self.ask_count != 0 {
            len += 1;
        }
        if self.min_count != 0 {
            len += 1;
        }
        if self.prepare_gas != 0 {
            len += 1;
        }
        if self.execute_gas != 0 {
            len += 1;
        }
        if !self.fee_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.OracleParams", len)?;
        if self.script_id != 0 {
            struct_ser
                .serialize_field("scriptId", ToString::to_string(&self.script_id).as_str())?;
        }
        if self.ask_count != 0 {
            struct_ser
                .serialize_field("askCount", ToString::to_string(&self.ask_count).as_str())?;
        }
        if self.min_count != 0 {
            struct_ser
                .serialize_field("minCount", ToString::to_string(&self.min_count).as_str())?;
        }
        if self.prepare_gas != 0 {
            struct_ser.serialize_field(
                "prepareGas",
                ToString::to_string(&self.prepare_gas).as_str(),
            )?;
        }
        if self.execute_gas != 0 {
            struct_ser.serialize_field(
                "executeGas",
                ToString::to_string(&self.execute_gas).as_str(),
            )?;
        }
        if !self.fee_amount.is_empty() {
            struct_ser.serialize_field("feeAmount", &self.fee_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OracleParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "script_id",
            "scriptId",
            "ask_count",
            "askCount",
            "min_count",
            "minCount",
            "prepare_gas",
            "prepareGas",
            "execute_gas",
            "executeGas",
            "fee_amount",
            "feeAmount",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScriptId,
            AskCount,
            MinCount,
            PrepareGas,
            ExecuteGas,
            FeeAmount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "scriptId" | "script_id" => Ok(GeneratedField::ScriptId),
                            "askCount" | "ask_count" => Ok(GeneratedField::AskCount),
                            "minCount" | "min_count" => Ok(GeneratedField::MinCount),
                            "prepareGas" | "prepare_gas" => Ok(GeneratedField::PrepareGas),
                            "executeGas" | "execute_gas" => Ok(GeneratedField::ExecuteGas),
                            "feeAmount" | "fee_amount" => Ok(GeneratedField::FeeAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OracleParams;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.OracleParams")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<OracleParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut script_id__ = None;
                let mut ask_count__ = None;
                let mut min_count__ = None;
                let mut prepare_gas__ = None;
                let mut execute_gas__ = None;
                let mut fee_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ScriptId => {
                            if script_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptId"));
                            }
                            script_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AskCount => {
                            if ask_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askCount"));
                            }
                            ask_count__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinCount => {
                            if min_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minCount"));
                            }
                            min_count__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PrepareGas => {
                            if prepare_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prepareGas"));
                            }
                            prepare_gas__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ExecuteGas => {
                            if execute_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executeGas"));
                            }
                            execute_gas__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FeeAmount => {
                            if fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeAmount"));
                            }
                            fee_amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OracleParams {
                    script_id: script_id__.unwrap_or_default(),
                    ask_count: ask_count__.unwrap_or_default(),
                    min_count: min_count__.unwrap_or_default(),
                    prepare_gas: prepare_gas__.unwrap_or_default(),
                    execute_gas: execute_gas__.unwrap_or_default(),
                    fee_amount: fee_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.OracleParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OracleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.oracle_script_id != 0 {
            len += 1;
        }
        if self.call_data.is_some() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.OracleRequest", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if self.oracle_script_id != 0 {
            struct_ser.serialize_field(
                "oracleScriptId",
                ToString::to_string(&self.oracle_script_id).as_str(),
            )?;
        }
        if let Some(v) = self.call_data.as_ref() {
            struct_ser.serialize_field("callData", v)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OracleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "oracle_script_id",
            "oracleScriptId",
            "call_data",
            "callData",
            "client_id",
            "clientId",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            OracleScriptId,
            CallData,
            ClientId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "oracleScriptId" | "oracle_script_id" => {
                                Ok(GeneratedField::OracleScriptId)
                            }
                            "callData" | "call_data" => Ok(GeneratedField::CallData),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OracleRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.OracleRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<OracleRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut oracle_script_id__ = None;
                let mut call_data__ = None;
                let mut client_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OracleScriptId => {
                            if oracle_script_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleScriptId"));
                            }
                            oracle_script_id__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CallData => {
                            if call_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callData"));
                            }
                            call_data__ = map.next_value()?;
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OracleRequest {
                    id: id__.unwrap_or_default(),
                    oracle_script_id: oracle_script_id__.unwrap_or_default(),
                    call_data: call_data__,
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.OracleRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for oracle_request::CallData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.application.is_empty() {
            len += 1;
        }
        if !self.call_data.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.OracleRequest.CallData", len)?;
        if !self.application.is_empty() {
            struct_ser.serialize_field("application", &self.application)?;
        }
        if !self.call_data.is_empty() {
            struct_ser.serialize_field("callData", &self.call_data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for oracle_request::CallData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["application", "call_data", "callData"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Application,
            CallData,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "application" => Ok(GeneratedField::Application),
                            "callData" | "call_data" => Ok(GeneratedField::CallData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = oracle_request::CallData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.OracleRequest.CallData")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<oracle_request::CallData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut application__ = None;
                let mut call_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Application => {
                            if application__.is_some() {
                                return Err(serde::de::Error::duplicate_field("application"));
                            }
                            application__ = Some(map.next_value()?);
                        }
                        GeneratedField::CallData => {
                            if call_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callData"));
                            }
                            call_data__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(oracle_request::CallData {
                    application: application__.unwrap_or_default(),
                    call_data: call_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.OracleRequest.CallData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nickname.is_some() {
            len += 1;
        }
        if self.dtag.is_some() {
            len += 1;
        }
        if self.bio.is_some() {
            len += 1;
        }
        if self.oracle.is_some() {
            len += 1;
        }
        if self.app_links.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.Params", len)?;
        if let Some(v) = self.nickname.as_ref() {
            struct_ser.serialize_field("nickname", v)?;
        }
        if let Some(v) = self.dtag.as_ref() {
            struct_ser.serialize_field("dtag", v)?;
        }
        if let Some(v) = self.bio.as_ref() {
            struct_ser.serialize_field("bio", v)?;
        }
        if let Some(v) = self.oracle.as_ref() {
            struct_ser.serialize_field("oracle", v)?;
        }
        if let Some(v) = self.app_links.as_ref() {
            struct_ser.serialize_field("appLinks", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nickname", "dtag", "bio", "oracle", "app_links", "appLinks"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nickname,
            Dtag,
            Bio,
            Oracle,
            AppLinks,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nickname" => Ok(GeneratedField::Nickname),
                            "dtag" => Ok(GeneratedField::Dtag),
                            "bio" => Ok(GeneratedField::Bio),
                            "oracle" => Ok(GeneratedField::Oracle),
                            "appLinks" | "app_links" => Ok(GeneratedField::AppLinks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nickname__ = None;
                let mut dtag__ = None;
                let mut bio__ = None;
                let mut oracle__ = None;
                let mut app_links__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Nickname => {
                            if nickname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickname"));
                            }
                            nickname__ = map.next_value()?;
                        }
                        GeneratedField::Dtag => {
                            if dtag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dtag"));
                            }
                            dtag__ = map.next_value()?;
                        }
                        GeneratedField::Bio => {
                            if bio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bio"));
                            }
                            bio__ = map.next_value()?;
                        }
                        GeneratedField::Oracle => {
                            if oracle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracle"));
                            }
                            oracle__ = map.next_value()?;
                        }
                        GeneratedField::AppLinks => {
                            if app_links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appLinks"));
                            }
                            app_links__ = map.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    nickname: nickname__,
                    dtag: dtag__,
                    bio: bio__,
                    oracle: oracle__,
                    app_links: app_links__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Pictures {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.profile.is_empty() {
            len += 1;
        }
        if !self.cover.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.Pictures", len)?;
        if !self.profile.is_empty() {
            struct_ser.serialize_field("profile", &self.profile)?;
        }
        if !self.cover.is_empty() {
            struct_ser.serialize_field("cover", &self.cover)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Pictures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["profile", "cover"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Profile,
            Cover,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "profile" => Ok(GeneratedField::Profile),
                            "cover" => Ok(GeneratedField::Cover),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Pictures;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Pictures")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Pictures, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut profile__ = None;
                let mut cover__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Profile => {
                            if profile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            profile__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cover => {
                            if cover__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cover"));
                            }
                            cover__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Pictures {
                    profile: profile__.unwrap_or_default(),
                    cover: cover__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.Pictures", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Profile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        if !self.dtag.is_empty() {
            len += 1;
        }
        if !self.nickname.is_empty() {
            len += 1;
        }
        if !self.bio.is_empty() {
            len += 1;
        }
        if self.pictures.is_some() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.Profile", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        if !self.dtag.is_empty() {
            struct_ser.serialize_field("dtag", &self.dtag)?;
        }
        if !self.nickname.is_empty() {
            struct_ser.serialize_field("nickname", &self.nickname)?;
        }
        if !self.bio.is_empty() {
            struct_ser.serialize_field("bio", &self.bio)?;
        }
        if let Some(v) = self.pictures.as_ref() {
            struct_ser.serialize_field("pictures", v)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Profile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
            "dtag",
            "nickname",
            "bio",
            "pictures",
            "creation_date",
            "creationDate",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
            Dtag,
            Nickname,
            Bio,
            Pictures,
            CreationDate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "account" => Ok(GeneratedField::Account),
                            "dtag" => Ok(GeneratedField::Dtag),
                            "nickname" => Ok(GeneratedField::Nickname),
                            "bio" => Ok(GeneratedField::Bio),
                            "pictures" => Ok(GeneratedField::Pictures),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Profile;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Profile")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Profile, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut dtag__ = None;
                let mut nickname__ = None;
                let mut bio__ = None;
                let mut pictures__ = None;
                let mut creation_date__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map.next_value()?;
                        }
                        GeneratedField::Dtag => {
                            if dtag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dtag"));
                            }
                            dtag__ = Some(map.next_value()?);
                        }
                        GeneratedField::Nickname => {
                            if nickname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickname"));
                            }
                            nickname__ = Some(map.next_value()?);
                        }
                        GeneratedField::Bio => {
                            if bio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bio"));
                            }
                            bio__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pictures => {
                            if pictures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pictures"));
                            }
                            pictures__ = map.next_value()?;
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map.next_value()?;
                        }
                    }
                }
                Ok(Profile {
                    account: account__,
                    dtag: dtag__.unwrap_or_default(),
                    nickname: nickname__.unwrap_or_default(),
                    bio: bio__.unwrap_or_default(),
                    pictures: pictures__,
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.Profile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Proof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pub_key.is_some() {
            len += 1;
        }
        if self.signature.is_some() {
            len += 1;
        }
        if !self.plain_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.Proof", len)?;
        if let Some(v) = self.pub_key.as_ref() {
            struct_ser.serialize_field("pubKey", v)?;
        }
        if let Some(v) = self.signature.as_ref() {
            struct_ser.serialize_field("signature", v)?;
        }
        if !self.plain_text.is_empty() {
            struct_ser.serialize_field("plainText", &self.plain_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Proof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pub_key", "pubKey", "signature", "plain_text", "plainText"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PubKey,
            Signature,
            PlainText,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "signature" => Ok(GeneratedField::Signature),
                            "plainText" | "plain_text" => Ok(GeneratedField::PlainText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Proof;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Proof")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Proof, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pub_key__ = None;
                let mut signature__ = None;
                let mut plain_text__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ = map.next_value()?;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = map.next_value()?;
                        }
                        GeneratedField::PlainText => {
                            if plain_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plainText"));
                            }
                            plain_text__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Proof {
                    pub_key: pub_key__,
                    signature: signature__,
                    plain_text: plain_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.Proof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryApplicationLinkByClientIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkByClientIDRequest",
            len,
        )?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryApplicationLinkByClientIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["client_id", "clientId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryApplicationLinkByClientIdRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.QueryApplicationLinkByClientIDRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryApplicationLinkByClientIdRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryApplicationLinkByClientIdRequest {
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkByClientIDRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryApplicationLinkByClientIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.link.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkByClientIDResponse",
            len,
        )?;
        if let Some(v) = self.link.as_ref() {
            struct_ser.serialize_field("link", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryApplicationLinkByClientIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["link"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Link,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "link" => Ok(GeneratedField::Link),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryApplicationLinkByClientIdResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.QueryApplicationLinkByClientIDResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryApplicationLinkByClientIdResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut link__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Link => {
                            if link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            link__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryApplicationLinkByClientIdResponse { link: link__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkByClientIDResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryApplicationLinkOwnersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.application.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.profiles.v3.QueryApplicationLinkOwnersRequest", len)?;
        if !self.application.is_empty() {
            struct_ser.serialize_field("application", &self.application)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryApplicationLinkOwnersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["application", "username", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Application,
            Username,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "application" => Ok(GeneratedField::Application),
                            "username" => Ok(GeneratedField::Username),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryApplicationLinkOwnersRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryApplicationLinkOwnersRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryApplicationLinkOwnersRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut application__ = None;
                let mut username__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Application => {
                            if application__.is_some() {
                                return Err(serde::de::Error::duplicate_field("application"));
                            }
                            application__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryApplicationLinkOwnersRequest {
                    application: application__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkOwnersRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryApplicationLinkOwnersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owners.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("desmos.profiles.v3.QueryApplicationLinkOwnersResponse", len)?;
        if !self.owners.is_empty() {
            struct_ser.serialize_field("owners", &self.owners)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryApplicationLinkOwnersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["owners", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owners,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "owners" => Ok(GeneratedField::Owners),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryApplicationLinkOwnersResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryApplicationLinkOwnersResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryApplicationLinkOwnersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owners__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owners => {
                            if owners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owners"));
                            }
                            owners__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryApplicationLinkOwnersResponse {
                    owners: owners__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkOwnersResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for query_application_link_owners_response::ApplicationLinkOwnerDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.application.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkOwnersResponse.ApplicationLinkOwnerDetails",
            len,
        )?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.application.is_empty() {
            struct_ser.serialize_field("application", &self.application)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de>
    for query_application_link_owners_response::ApplicationLinkOwnerDetails
{
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "application", "username"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Application,
            Username,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "application" => Ok(GeneratedField::Application),
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = query_application_link_owners_response::ApplicationLinkOwnerDetails;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct desmos.profiles.v3.QueryApplicationLinkOwnersResponse.ApplicationLinkOwnerDetails",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<
                query_application_link_owners_response::ApplicationLinkOwnerDetails,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut application__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::Application => {
                            if application__.is_some() {
                                return Err(serde::de::Error::duplicate_field("application"));
                            }
                            application__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(
                    query_application_link_owners_response::ApplicationLinkOwnerDetails {
                        user: user__.unwrap_or_default(),
                        application: application__.unwrap_or_default(),
                        username: username__.unwrap_or_default(),
                    },
                )
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryApplicationLinkOwnersResponse.ApplicationLinkOwnerDetails",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryApplicationLinksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.application.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryApplicationLinksRequest", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.application.is_empty() {
            struct_ser.serialize_field("application", &self.application)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryApplicationLinksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "application", "username", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Application,
            Username,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "application" => Ok(GeneratedField::Application),
                            "username" => Ok(GeneratedField::Username),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryApplicationLinksRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryApplicationLinksRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryApplicationLinksRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut application__ = None;
                let mut username__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::Application => {
                            if application__.is_some() {
                                return Err(serde::de::Error::duplicate_field("application"));
                            }
                            application__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryApplicationLinksRequest {
                    user: user__.unwrap_or_default(),
                    application: application__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryApplicationLinksRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryApplicationLinksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.links.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryApplicationLinksResponse", len)?;
        if !self.links.is_empty() {
            struct_ser.serialize_field("links", &self.links)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryApplicationLinksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["links", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Links,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "links" => Ok(GeneratedField::Links),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryApplicationLinksResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryApplicationLinksResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryApplicationLinksResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut links__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Links => {
                            if links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryApplicationLinksResponse {
                    links: links__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryApplicationLinksResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryChainLinkOwnersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_name.is_empty() {
            len += 1;
        }
        if !self.target.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryChainLinkOwnersRequest", len)?;
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryChainLinkOwnersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_name", "chainName", "target", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainName,
            Target,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            "target" => Ok(GeneratedField::Target),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryChainLinkOwnersRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryChainLinkOwnersRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryChainLinkOwnersRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_name__ = None;
                let mut target__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryChainLinkOwnersRequest {
                    chain_name: chain_name__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryChainLinkOwnersRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryChainLinkOwnersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owners.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryChainLinkOwnersResponse", len)?;
        if !self.owners.is_empty() {
            struct_ser.serialize_field("owners", &self.owners)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryChainLinkOwnersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["owners", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owners,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "owners" => Ok(GeneratedField::Owners),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryChainLinkOwnersResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryChainLinkOwnersResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryChainLinkOwnersResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owners__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owners => {
                            if owners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owners"));
                            }
                            owners__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryChainLinkOwnersResponse {
                    owners: owners__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryChainLinkOwnersResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for query_chain_link_owners_response::ChainLinkOwnerDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.chain_name.is_empty() {
            len += 1;
        }
        if !self.target.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryChainLinkOwnersResponse.ChainLinkOwnerDetails",
            len,
        )?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for query_chain_link_owners_response::ChainLinkOwnerDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "chain_name", "chainName", "target"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            ChainName,
            Target,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = query_chain_link_owners_response::ChainLinkOwnerDetails;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct desmos.profiles.v3.QueryChainLinkOwnersResponse.ChainLinkOwnerDetails",
                )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<
                query_chain_link_owners_response::ChainLinkOwnerDetails,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut chain_name__ = None;
                let mut target__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(query_chain_link_owners_response::ChainLinkOwnerDetails {
                    user: user__.unwrap_or_default(),
                    chain_name: chain_name__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryChainLinkOwnersResponse.ChainLinkOwnerDetails",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryChainLinksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.chain_name.is_empty() {
            len += 1;
        }
        if !self.target.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryChainLinksRequest", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryChainLinksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user", "chain_name", "chainName", "target", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            ChainName,
            Target,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            "target" => Ok(GeneratedField::Target),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryChainLinksRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryChainLinksRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryChainLinksRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut chain_name__ = None;
                let mut target__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryChainLinksRequest {
                    user: user__.unwrap_or_default(),
                    chain_name: chain_name__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryChainLinksRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryChainLinksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.links.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryChainLinksResponse", len)?;
        if !self.links.is_empty() {
            struct_ser.serialize_field("links", &self.links)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryChainLinksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["links", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Links,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "links" => Ok(GeneratedField::Links),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryChainLinksResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryChainLinksResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryChainLinksResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut links__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Links => {
                            if links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryChainLinksResponse {
                    links: links__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryChainLinksResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryDefaultExternalAddressesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.chain_name.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryDefaultExternalAddressesRequest",
            len,
        )?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDefaultExternalAddressesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["owner", "chain_name", "chainName", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            ChainName,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "owner" => Ok(GeneratedField::Owner),
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDefaultExternalAddressesRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.QueryDefaultExternalAddressesRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDefaultExternalAddressesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut chain_name__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryDefaultExternalAddressesRequest {
                    owner: owner__.unwrap_or_default(),
                    chain_name: chain_name__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryDefaultExternalAddressesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryDefaultExternalAddressesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.links.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryDefaultExternalAddressesResponse",
            len,
        )?;
        if !self.links.is_empty() {
            struct_ser.serialize_field("links", &self.links)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDefaultExternalAddressesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["links", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Links,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "links" => Ok(GeneratedField::Links),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDefaultExternalAddressesResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.QueryDefaultExternalAddressesResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDefaultExternalAddressesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut links__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Links => {
                            if links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryDefaultExternalAddressesResponse {
                    links: links__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryDefaultExternalAddressesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryIncomingDTagTransferRequestsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.receiver.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryIncomingDTagTransferRequestsRequest",
            len,
        )?;
        if !self.receiver.is_empty() {
            struct_ser.serialize_field("receiver", &self.receiver)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIncomingDTagTransferRequestsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["receiver", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Receiver,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "receiver" => Ok(GeneratedField::Receiver),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryIncomingDTagTransferRequestsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .write_str("struct desmos.profiles.v3.QueryIncomingDTagTransferRequestsRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryIncomingDTagTransferRequestsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut receiver__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Receiver => {
                            if receiver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receiver"));
                            }
                            receiver__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryIncomingDTagTransferRequestsRequest {
                    receiver: receiver__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryIncomingDTagTransferRequestsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryIncomingDTagTransferRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct(
            "desmos.profiles.v3.QueryIncomingDTagTransferRequestsResponse",
            len,
        )?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIncomingDTagTransferRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["requests", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
            Pagination,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryIncomingDTagTransferRequestsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "struct desmos.profiles.v3.QueryIncomingDTagTransferRequestsResponse",
                )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryIncomingDTagTransferRequestsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryIncomingDTagTransferRequestsResponse {
                    requests: requests__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryIncomingDTagTransferRequestsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryParamsRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryParamsResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryProfileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryProfileRequest", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProfileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryProfileRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryProfileRequest")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProfileRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryProfileRequest {
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryProfileRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for QueryProfileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.profile.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.QueryProfileResponse", len)?;
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProfileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["profile"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Profile,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "profile" => Ok(GeneratedField::Profile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryProfileResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.QueryProfileResponse")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProfileResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut profile__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Profile => {
                            if profile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            profile__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProfileResponse { profile: profile__ })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.QueryProfileResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Result {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sum.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v3.Result", len)?;
        if let Some(v) = self.sum.as_ref() {
            match v {
                result::Sum::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                result::Sum::Failed(v) => {
                    struct_ser.serialize_field("failed", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Result {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["success", "failed"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            Failed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            "failed" => Ok(GeneratedField::Failed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Result;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Result")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Result, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sum__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            sum__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(result::Sum::Success);
                        }
                        GeneratedField::Failed => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            sum__ = map
                                .next_value::<::std::option::Option<_>>()?
                                .map(result::Sum::Failed);
                        }
                    }
                }
                Ok(Result { sum: sum__ })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v3.Result", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for result::Failed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.error.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.Result.Failed", len)?;
        if !self.error.is_empty() {
            struct_ser.serialize_field("error", &self.error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for result::Failed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["error"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = result::Failed;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Result.Failed")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<result::Failed, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(result::Failed {
                    error: error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.Result.Failed",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for result::Success {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.Result.Success", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for result::Success {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value", "signature"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Signature,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = result::Success;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.Result.Success")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<result::Success, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(result::Success {
                    value: value__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.Result.Success",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SignatureValueType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SIGNATURE_VALUE_TYPE_UNSPECIFIED",
            Self::Raw => "SIGNATURE_VALUE_TYPE_RAW",
            Self::CosmosDirect => "SIGNATURE_VALUE_TYPE_COSMOS_DIRECT",
            Self::CosmosAmino => "SIGNATURE_VALUE_TYPE_COSMOS_AMINO",
            Self::EvmPersonalSign => "SIGNATURE_VALUE_TYPE_EVM_PERSONAL_SIGN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SignatureValueType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGNATURE_VALUE_TYPE_UNSPECIFIED",
            "SIGNATURE_VALUE_TYPE_RAW",
            "SIGNATURE_VALUE_TYPE_COSMOS_DIRECT",
            "SIGNATURE_VALUE_TYPE_COSMOS_AMINO",
            "SIGNATURE_VALUE_TYPE_EVM_PERSONAL_SIGN",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignatureValueType;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(SignatureValueType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(SignatureValueType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SIGNATURE_VALUE_TYPE_UNSPECIFIED" => Ok(SignatureValueType::Unspecified),
                    "SIGNATURE_VALUE_TYPE_RAW" => Ok(SignatureValueType::Raw),
                    "SIGNATURE_VALUE_TYPE_COSMOS_DIRECT" => Ok(SignatureValueType::CosmosDirect),
                    "SIGNATURE_VALUE_TYPE_COSMOS_AMINO" => Ok(SignatureValueType::CosmosAmino),
                    "SIGNATURE_VALUE_TYPE_EVM_PERSONAL_SIGN" => {
                        Ok(SignatureValueType::EvmPersonalSign)
                    }
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SingleSignature {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_type != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v3.SingleSignature", len)?;
        if self.value_type != 0 {
            let v = SignatureValueType::from_i32(self.value_type).ok_or_else(|| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.value_type))
            })?;
            struct_ser.serialize_field("valueType", &v)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field(
                "signature",
                pbjson::private::base64::encode(&self.signature).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SingleSignature {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value_type", "valueType", "signature"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueType,
            Signature,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueType" | "value_type" => Ok(GeneratedField::ValueType),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SingleSignature;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v3.SingleSignature")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<SingleSignature, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value_type__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueType"));
                            }
                            value_type__ = Some(map.next_value::<SignatureValueType>()? as i32);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(SingleSignature {
                    value_type: value_type__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v3.SingleSignature",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
