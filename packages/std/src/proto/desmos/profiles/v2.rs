/// ApplicationLink contains the data of a link to a centralized application
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.ApplicationLink")]
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
}
/// Data contains the data associated to a specific user of a
/// generic centralized application
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.Data")]
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
#[proto_message(type_url = "/desmos.profiles.v2.OracleRequest")]
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
    #[proto_message(type_url = "/desmos.profiles.v2.OracleRequest.CallData")]
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
#[proto_message(type_url = "/desmos.profiles.v2.Result")]
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
    #[proto_message(type_url = "/desmos.profiles.v2.Result.Success")]
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
    #[proto_message(type_url = "/desmos.profiles.v2.Result.Failed")]
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
/// Profile represents a generic first on Desmos, containing the information of a
/// single user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.Profile")]
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
#[proto_message(type_url = "/desmos.profiles.v2.Pictures")]
pub struct Pictures {
    /// Profile contains the URL to the profile picture
    #[prost(string, tag = "1")]
    pub profile: ::prost::alloc::string::String,
    /// Cover contains the URL to the cover picture
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
}
/// ChainLink contains the data representing either an inter- or cross- chain
/// link
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.ChainLink")]
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
#[proto_message(type_url = "/desmos.profiles.v2.ChainConfig")]
pub struct ChainConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Proof contains all the data used to verify a signature when linking an
/// account to a profile
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.Proof")]
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
#[proto_message(type_url = "/desmos.profiles.v2.Bech32Address")]
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
#[proto_message(type_url = "/desmos.profiles.v2.Base58Address")]
pub struct Base58Address {
    /// Value contains the Base58-encoded address
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// HexAddress represents an Hex-encoded address
/// NOTE: Currently it only supports keccak256-uncompressed addresses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.HexAddress")]
pub struct HexAddress {
    /// Value represents the hex address value
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// Prefix represents the optional prefix used during address encoding (e.g.
    /// 0x)
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
}
/// SingleSignatureData is the signature data for a single signer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.SingleSignatureData")]
pub struct SingleSignatureData {
    /// Mode is the signing mode of the single signer
    #[prost(
        enumeration = "super::super::super::cosmos::tx::signing::v1beta1::SignMode",
        tag = "1"
    )]
    pub mode: i32,
    /// Signature is the raw signature bytes
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// MultiSignatureData is the signature data for a multisig public key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.MultiSignatureData")]
pub struct MultiSignatureData {
    /// Bitarray specifies which keys within the multisig are signing
    #[prost(message, optional, tag = "1")]
    pub bit_array: ::core::option::Option<
        super::super::super::cosmos::crypto::multisig::v1beta1::CompactBitArray,
    >,
    /// Signatures is the signatures of the multi-signature
    #[prost(message, repeated, tag = "2")]
    pub signatures: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// DTagTransferRequest represent a DTag transfer request between two users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.DTagTransferRequest")]
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
/// Params contains the parameters for the profiles module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub nickname: ::core::option::Option<NicknameParams>,
    #[prost(message, optional, tag = "2")]
    pub dtag: ::core::option::Option<DTagParams>,
    #[prost(message, optional, tag = "3")]
    pub bio: ::core::option::Option<BioParams>,
    #[prost(message, optional, tag = "4")]
    pub oracle: ::core::option::Option<OracleParams>,
}
/// NicknameParams defines the parameters related to the profiles nicknames
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.NicknameParams")]
pub struct NicknameParams {
    #[prost(bytes = "vec", tag = "1")]
    pub min_length: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub max_length: ::prost::alloc::vec::Vec<u8>,
}
/// DTagParams defines the parameters related to profile DTags
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.DTagParams")]
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
#[proto_message(type_url = "/desmos.profiles.v2.BioParams")]
pub struct BioParams {
    #[prost(bytes = "vec", tag = "3")]
    pub max_length: ::prost::alloc::vec::Vec<u8>,
}
/// OracleParams defines the parameters related to the oracle
/// that will be used to verify the ownership of a centralized
/// application account by a Desmos profile
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, schemars::JsonSchema, std_derive::CosmwasmExt)]
#[proto_message(type_url = "/desmos.profiles.v2.OracleParams")]
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
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v2.ApplicationLink", len)?;
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
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Data,
            State,
            OracleRequest,
            Result,
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
                            "data" => Ok(GeneratedField::Data),
                            "state" => Ok(GeneratedField::State),
                            "oracleRequest" | "oracle_request" => Ok(GeneratedField::OracleRequest),
                            "result" => Ok(GeneratedField::Result),
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
            type Value = ApplicationLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v2.ApplicationLink")
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
                    }
                }
                Ok(ApplicationLink {
                    user: user__.unwrap_or_default(),
                    data: data__,
                    state: state__.unwrap_or_default(),
                    oracle_request: oracle_request__,
                    result: result__,
                    creation_time: creation_time__,
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v2.ApplicationLink",
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
            serializer.serialize_struct("desmos.profiles.v2.Base58Address", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Base58Address")
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
            "desmos.profiles.v2.Base58Address",
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
            serializer.serialize_struct("desmos.profiles.v2.Bech32Address", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Bech32Address")
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
            "desmos.profiles.v2.Bech32Address",
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.BioParams", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.BioParams")
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
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
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
        deserializer.deserialize_struct("desmos.profiles.v2.BioParams", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.ChainConfig", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.ChainConfig")
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
        deserializer.deserialize_struct("desmos.profiles.v2.ChainConfig", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.ChainLink", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.ChainLink")
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
        deserializer.deserialize_struct("desmos.profiles.v2.ChainLink", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.DTagParams", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.DTagParams")
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
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
                                    .0,
                            );
                        }
                        GeneratedField::MaxLength => {
                            if max_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLength"));
                            }
                            max_length__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
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
        deserializer.deserialize_struct("desmos.profiles.v2.DTagParams", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("desmos.profiles.v2.DTagTransferRequest", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.DTagTransferRequest")
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
            "desmos.profiles.v2.DTagTransferRequest",
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.Data", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Data")
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
        deserializer.deserialize_struct("desmos.profiles.v2.Data", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.HexAddress", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.HexAddress")
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
        deserializer.deserialize_struct("desmos.profiles.v2.HexAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MultiSignatureData {
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
            serializer.serialize_struct("desmos.profiles.v2.MultiSignatureData", len)?;
        if let Some(v) = self.bit_array.as_ref() {
            struct_ser.serialize_field("bitArray", v)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MultiSignatureData {
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
            type Value = MultiSignatureData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v2.MultiSignatureData")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MultiSignatureData, V::Error>
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
                Ok(MultiSignatureData {
                    bit_array: bit_array__,
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v2.MultiSignatureData",
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
            serializer.serialize_struct("desmos.profiles.v2.NicknameParams", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.NicknameParams")
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
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
                                    .0,
                            );
                        }
                        GeneratedField::MaxLength => {
                            if max_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLength"));
                            }
                            max_length__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
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
            "desmos.profiles.v2.NicknameParams",
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.OracleParams", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.OracleParams")
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
        deserializer.deserialize_struct("desmos.profiles.v2.OracleParams", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("desmos.profiles.v2.OracleRequest", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.OracleRequest")
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
            "desmos.profiles.v2.OracleRequest",
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
            serializer.serialize_struct("desmos.profiles.v2.OracleRequest.CallData", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.OracleRequest.CallData")
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
            "desmos.profiles.v2.OracleRequest.CallData",
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.Params", len)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nickname", "dtag", "bio", "oracle"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nickname,
            Dtag,
            Bio,
            Oracle,
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
                formatter.write_str("struct desmos.profiles.v2.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nickname__ = None;
                let mut dtag__ = None;
                let mut bio__ = None;
                let mut oracle__ = None;
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
                    }
                }
                Ok(Params {
                    nickname: nickname__,
                    dtag: dtag__,
                    bio: bio__,
                    oracle: oracle__,
                })
            }
        }
        deserializer.deserialize_struct("desmos.profiles.v2.Params", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.Pictures", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Pictures")
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
        deserializer.deserialize_struct("desmos.profiles.v2.Pictures", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.Profile", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Profile")
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
        deserializer.deserialize_struct("desmos.profiles.v2.Profile", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.Proof", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Proof")
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
        deserializer.deserialize_struct("desmos.profiles.v2.Proof", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("desmos.profiles.v2.Result", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Result")
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
        deserializer.deserialize_struct("desmos.profiles.v2.Result", FIELDS, GeneratedVisitor)
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
            serializer.serialize_struct("desmos.profiles.v2.Result.Failed", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Result.Failed")
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
            "desmos.profiles.v2.Result.Failed",
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
            serializer.serialize_struct("desmos.profiles.v2.Result.Success", len)?;
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
                formatter.write_str("struct desmos.profiles.v2.Result.Success")
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
            "desmos.profiles.v2.Result.Success",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SingleSignatureData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mode != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("desmos.profiles.v2.SingleSignatureData", len)?;
        if self.mode != 0 {
            let v =
                super::super::super::cosmos::tx::signing::v1beta1::SignMode::from_i32(self.mode)
                    .ok_or_else(|| {
                        serde::ser::Error::custom(format!("Invalid variant {}", self.mode))
                    })?;
            struct_ser.serialize_field("mode", &v)?;
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
impl<'de> serde::Deserialize<'de> for SingleSignatureData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["mode", "signature"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mode,
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
                            "mode" => Ok(GeneratedField::Mode),
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
            type Value = SingleSignatureData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct desmos.profiles.v2.SingleSignatureData")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<SingleSignatureData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut mode__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(
                                map
                                    .next_value::<
                                        super::super::super::cosmos::tx::signing::v1beta1::SignMode,
                                    >()? as i32,
                            );
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()
                                    .unwrap_or(pbjson::private::BytesDeserialize(vec![]))
                                    .0,
                            );
                        }
                    }
                }
                Ok(SingleSignatureData {
                    mode: mode__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "desmos.profiles.v2.SingleSignatureData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
