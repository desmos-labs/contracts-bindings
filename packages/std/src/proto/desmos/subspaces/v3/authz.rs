/// GenericSubspaceAuthorization defines an authorization to perform any
/// operation only inside a specific subspace.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.subspaces.v3.authz.GenericSubspaceAuthorization")]
pub struct GenericSubspaceAuthorization {
    /// Ids of the subspaces inside which to grant the permission
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub subspaces_ids: ::prost::alloc::vec::Vec<u64>,
    /// Msg, identified by it's type URL, to grant unrestricted permissions to
    /// execute within the subspace
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
}
