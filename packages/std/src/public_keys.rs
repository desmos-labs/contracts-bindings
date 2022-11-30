/// PubKey is an ed25519 public key for handling Tendermint keys in SDK.
/// It's needed for Any serialization and SDK compatibility.
/// It must not be used in a non Tendermint key context because it doesn't implement
/// ADR-28. Nevertheless, you will like to use ed25519 in app user level
/// then you must create a new proto message and follow ADR-28 for Address construction.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.crypto.ed25519.PubKey")]
pub struct Ed25519PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}

/// PubKey defines a secp256k1 public key
/// Key is the compressed form of the pubkey. The first byte depends is a 0x02 byte
/// if the y-coordinate is the lexicographically largest of the two associated with
/// the x-coordinate. Otherwise the first byte is a 0x03.
/// This prefix is followed with the x-coordinate.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.crypto.secp256k1.PubKey")]
pub struct Secp256k1PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}

/// PubKey defines a secp256r1 ECDSA public key.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.crypto.secp256r1.PubKey")]
pub struct Secp256r1PublicKey {
    /// Point on secp256r1 curve in a compressed representation as specified in section
    /// 4.3.6 of ANSI X9.62: https://webstore.ansi.org/standards/ascx9/ansix9621998
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
