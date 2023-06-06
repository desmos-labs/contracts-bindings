/// PubKey defines a secp256r1 ECDSA public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt
)]
#[proto_message(type_url = "/cosmos.crypto.secp256r1.PubKey")]
#[serde(rename_all = "snake_case")]
pub struct PubKey {
    /// Point on secp256r1 curve in a compressed representation as specified in section
    /// 4.3.6 of ANSI X9.62: <https://webstore.ansi.org/standards/ascx9/ansix9621998>
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// PrivKey defines a secp256r1 ECDSA private key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    desmos_std_derive::CosmwasmExt
)]
#[proto_message(type_url = "/cosmos.crypto.secp256r1.PrivKey")]
#[serde(rename_all = "snake_case")]
pub struct PrivKey {
    /// secret number serialized using big-endian encoding
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub secret: ::prost::alloc::vec::Vec<u8>,
}
