/// Subspace contains all the data of a Desmos subspace
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
#[proto_message(type_url = "/desmos.subspaces.v1.Subspace")]
#[serde(rename_all = "snake_case")]
pub struct Subspace {
    /// Unique id that identifies the subspace
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// Human-readable name of the subspace
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Optional description of this subspace
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Represents the account that is associated with the subspace and
    /// should be used to connect external applications to verify this subspace
    #[prost(string, tag = "4")]
    pub treasury: ::prost::alloc::string::String,
    /// Address of the user that owns the subspace
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
    /// Address of the subspace creator
    #[prost(string, tag = "6")]
    pub creator: ::prost::alloc::string::String,
    /// the creation time of the subspace
    #[prost(message, optional, tag = "7")]
    pub creation_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// UserGroup represents a group of users
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
#[proto_message(type_url = "/desmos.subspaces.v1.UserGroup")]
#[serde(rename_all = "snake_case")]
pub struct UserGroup {
    /// ID of the subspace inside which this group exists
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Unique id that identifies the group
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// Human-readable name of the user group
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Optional description of this group
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Permissions that will be granted to all the users part of this group
    #[prost(uint32, tag = "5")]
    pub permissions: u32,
}
/// PermissionDetail contains the details data of a permission
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
#[proto_message(type_url = "/desmos.subspaces.v1.PermissionDetail")]
#[serde(rename_all = "snake_case")]
pub struct PermissionDetail {
    /// sum is the oneof that specifies whether this represents a user or
    /// group permission detail
    #[prost(oneof = "permission_detail::Sum", tags = "1, 2")]
    #[serde(flatten)]
    pub sum: ::core::option::Option<permission_detail::Sum>,
}
/// Nested message and enum types in `PermissionDetail`.
pub mod permission_detail {
    /// User is a permission that has been set to a specific user
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
    #[proto_message(type_url = "/desmos.subspaces.v1.PermissionDetail.User")]
    #[serde(rename_all = "snake_case")]
    pub struct User {
        /// User for which the permission was set
        #[prost(string, tag = "1")]
        pub user: ::prost::alloc::string::String,
        /// Permission set to the user
        #[prost(uint32, tag = "2")]
        pub permission: u32,
    }
    /// Group is a permission that has been set to a user group
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
    #[proto_message(type_url = "/desmos.subspaces.v1.PermissionDetail.Group")]
    #[serde(rename_all = "snake_case")]
    pub struct Group {
        /// Unique id of the group
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
        /// Permission set to the group
        #[prost(uint32, tag = "2")]
        pub permission: u32,
    }
    /// sum is the oneof that specifies whether this represents a user or
    /// group permission detail
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone, PartialEq, ::prost::Oneof, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
    )]
    #[serde(rename_all = "snake_case")]
    pub enum Sum {
        /// User represents a user permission
        #[prost(message, tag = "1")]
        User(User),
        /// Group represents a group permission
        #[prost(message, tag = "2")]
        Group(Group),
    }
}