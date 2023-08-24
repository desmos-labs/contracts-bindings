pub mod authz;
/// Subspace contains all the data of a Desmos subspace
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
#[proto_message(type_url = "/desmos.subspaces.v3.Subspace")]
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
    /// Represents the treasury account that is associated with the subspace
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
    /// List of fee token denoms with default minimum gas prices allowed inside the
    /// subspace
    #[prost(message, repeated, tag = "8")]
    pub additional_fee_tokens:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Section contains the data of a single subspace section
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
#[proto_message(type_url = "/desmos.subspaces.v3.Section")]
#[serde(rename_all = "snake_case")]
pub struct Section {
    /// Id of the subspace inside which the section exists
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Unique id of the section within the subspace
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// (optional) Id of the parent section
    #[prost(uint32, tag = "3")]
    pub parent_id: u32,
    /// Name of the section within the subspace
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// (optional) Description of the section
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
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
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.subspaces.v3.UserGroup")]
#[serde(rename_all = "snake_case")]
pub struct UserGroup {
    /// ID of the subspace inside which this group exists
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// (optional) Id of the section inside which this group is valid
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// Unique id that identifies the group
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// Human-readable name of the user group
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Optional description of this group
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Permissions that will be granted to all the users part of this group
    #[prost(string, repeated, tag = "6")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// UserPermission represents a single Access Control List entry
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
#[proto_message(type_url = "/desmos.subspaces.v3.UserPermission")]
#[serde(rename_all = "snake_case")]
pub struct UserPermission {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Grant represents a grant to a user or a group
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
#[proto_message(type_url = "/desmos.subspaces.v3.Grant")]
#[serde(rename_all = "snake_case")]
pub struct Grant {
    /// Id of the subspace inside which the user was granted the allowance
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of the user that granted the allowance
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    /// Target to which the allowance has been granted
    #[prost(message, optional, tag = "3")]
    pub grantee: ::core::option::Option<crate::shim::Any>,
    /// Allowance can be any allowance type implementing the FeeAllowanceI
    /// interface
    #[prost(message, optional, tag = "4")]
    pub allowance: ::core::option::Option<crate::shim::Any>,
}
/// UserGrantee contains the target of a grant about a user
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
#[proto_message(type_url = "/desmos.subspaces.v3.UserGrantee")]
#[serde(rename_all = "snake_case")]
pub struct UserGrantee {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
/// GroupGrantee contains the target of a grant about a group
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
#[proto_message(type_url = "/desmos.subspaces.v3.GroupGrantee")]
#[serde(rename_all = "snake_case")]
pub struct GroupGrantee {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
/// GenesisState contains the data of the genesis state for the subspaces module
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
#[proto_message(type_url = "/desmos.subspaces.v3.GenesisState")]
#[serde(rename_all = "snake_case")]
pub struct GenesisState {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub initial_subspace_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub subspaces_data: ::prost::alloc::vec::Vec<SubspaceData>,
    #[prost(message, repeated, tag = "3")]
    pub subspaces: ::prost::alloc::vec::Vec<Subspace>,
    #[prost(message, repeated, tag = "4")]
    pub sections: ::prost::alloc::vec::Vec<Section>,
    #[prost(message, repeated, tag = "5")]
    pub user_permissions: ::prost::alloc::vec::Vec<UserPermission>,
    #[prost(message, repeated, tag = "6")]
    pub user_groups: ::prost::alloc::vec::Vec<UserGroup>,
    #[prost(message, repeated, tag = "7")]
    pub user_groups_members: ::prost::alloc::vec::Vec<UserGroupMemberEntry>,
    #[prost(message, repeated, tag = "8")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
}
/// SubspaceData contains the genesis data for a single subspace
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
#[proto_message(type_url = "/desmos.subspaces.v3.SubspaceData")]
#[serde(rename_all = "snake_case")]
pub struct SubspaceData {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    #[prost(uint32, tag = "2")]
    pub next_group_id: u32,
    #[prost(uint32, tag = "3")]
    pub next_section_id: u32,
}
/// UserGroupMemberEntry contains the details of a user group member
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
#[proto_message(type_url = "/desmos.subspaces.v3.UserGroupMemberEntry")]
#[serde(rename_all = "snake_case")]
pub struct UserGroupMemberEntry {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
}
/// MsgCreateSubspace represents the message used to create a subspace
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgCreateSubspace")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateSubspace {
    /// Name of the subspace
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// (optional) Description of the subspace
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// (optional) Owner of this subspace. If not specified, the creator will be
    /// the default owner.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    /// Address creating the subspace
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgCreateSubspaceResponse defines the Msg/CreateSubspace response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgCreateSubspaceResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateSubspaceResponse {
    /// Id of the newly created subspace id
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// MsgEditSubspace represents the message used to edit a subspace fields
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgEditSubspace")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditSubspace {
    /// Id of the subspace to edit
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// New name of the subspace. If it shouldn't be changed, use \[do-not-modify\]
    /// instead.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// New description of the subspace. If it shouldn't be changed, use
    /// \[do-not-modify\] instead.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// New owner of the subspace. If it shouldn't be changed, use \[do-not-modify\]
    /// instead.
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
    /// Address of the user editing the subspace
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgEditSubspaceResponse defines the Msg/EditSubspace response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgEditSubspaceResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditSubspaceResponse {}
/// MsgDeleteSubspace represents the message used to delete a subspace
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgDeleteSubspace")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteSubspace {
    /// Id of the subspace to delete
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of the user deleting the subspace
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgDeleteSubspaceResponse defines the Msg/DeleteSubspace response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgDeleteSubspaceResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteSubspaceResponse {}
/// MsgCreateSection represents the message to be used when creating a subspace
/// section
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgCreateSection")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateSection {
    /// Id of the subspace inside which the section will be placed
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Name of the section to be created
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// (optional) Description of the section
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// (optional) Id of the parent section
    #[prost(uint32, tag = "4")]
    pub parent_id: u32,
    /// User creating the section
    #[prost(string, tag = "5")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgCreateSectionResponse represents the Msg/CreateSection response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgCreateSectionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateSectionResponse {
    /// Id of the newly created section
    #[prost(uint32, tag = "1")]
    pub section_id: u32,
}
/// MsgEditSection represents the message to be used when editing a subspace
/// section
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgEditSection")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditSection {
    /// Id of the subspace inside which the section to be edited is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section to be edited
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// (optional) New name of the section
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// (optional) New description of the section
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// User editing the section
    #[prost(string, tag = "5")]
    pub editor: ::prost::alloc::string::String,
}
/// MsgEditSectionResponse represents the Msg/EditSection response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgEditSectionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditSectionResponse {}
/// MsgMoveSection represents the message to be used when moving a section to
/// another parent
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgMoveSection")]
#[serde(rename_all = "snake_case")]
pub struct MsgMoveSection {
    /// Id of the subspace inside which the section lies
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section to be moved
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// Id of the new parent
    #[prost(uint32, tag = "3")]
    pub new_parent_id: u32,
    /// Signer of the message
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgMoveSectionResponse
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgMoveSectionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMoveSectionResponse {}
/// MsgDeleteSection represents the message to be used when deleting a section
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgDeleteSection")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteSection {
    /// Id of the subspace inside which the section to be deleted is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section to delete
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// User deleting the section
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgDeleteSectionResponse represents the Msg/DeleteSection response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgDeleteSectionResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteSectionResponse {}
/// MsgCreateUserGroup represents the message used to create a user group
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgCreateUserGroup")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateUserGroup {
    /// Id of the subspace inside which the group will be created
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// (optional) Id of the section inside which the group will be created
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// Name of the group
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// (optional) Description of the group
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Default permissions to be applied to the group
    #[prost(string, repeated, tag = "5")]
    pub default_permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Initial members to be put inside the group
    #[prost(string, repeated, tag = "6")]
    pub initial_members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Creator of the group
    #[prost(string, tag = "7")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgCreateUserGroupResponse defines the Msg/CreateUserGroup response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgCreateUserGroupResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateUserGroupResponse {
    #[prost(uint32, tag = "1")]
    pub group_id: u32,
}
/// MsgEditUserGroup represents the message used to edit a user group
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgEditUserGroup")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditUserGroup {
    /// Id of the subspace inside which the group to be edited is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the group to be edited
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// (optional) New name of the group
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// (optional) New description of the group
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// User editing the group
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgEditUserGroupResponse defines the Msg/EditUserGroup response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgEditUserGroupResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgEditUserGroupResponse {}
/// MsgMoveUserGroup represents the message used to move one user group from a
/// section to another
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgMoveUserGroup")]
#[serde(rename_all = "snake_case")]
pub struct MsgMoveUserGroup {
    /// Id of the subspace inside which the group to move is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the group to be moved
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// Id of the new section where to move the group
    #[prost(uint32, tag = "3")]
    pub new_section_id: u32,
    /// User signing the message
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgMoveUserGroupResponse defines the Msg/MoveUserGroup response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgMoveUserGroupResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMoveUserGroupResponse {}
/// MsgSetUserGroupPermissions represents the message used to set the permissions
/// of a user group
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgSetUserGroupPermissions")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetUserGroupPermissions {
    /// Id of the subspace inside which the group is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the group for which to set the new permissions
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// New permissions to be set to the group
    #[prost(string, repeated, tag = "3")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// User setting the new permissions
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSetUserGroupPermissionsResponse defines the
/// Msg/SetUserGroupPermissionsResponse response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgSetUserGroupPermissionsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetUserGroupPermissionsResponse {}
/// MsgDeleteUserGroup represents the message used to delete a user group
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgDeleteUserGroup")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteUserGroup {
    /// Id of the subspace inside which the group to delete is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the group to be deleted
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// User deleting the group
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgDeleteUserGroupResponse defines the Msg/DeleteUserGroup response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgDeleteUserGroupResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteUserGroupResponse {}
/// MsgAddUserToUserGroup represents the message used to add a user to a user
/// group
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgAddUserToUserGroup")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddUserToUserGroup {
    /// Id of the subspace inside which the group is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the group to which to add the user
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// User to be added to the group
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    /// User signing the message
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAddUserToUserGroupResponse defines the Msg/AddUserToUserGroupResponse
/// response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgAddUserToUserGroupResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddUserToUserGroupResponse {}
/// MsgRemoveUserFromUserGroup represents the message used to remove a user from
/// a user group
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgRemoveUserFromUserGroup")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveUserFromUserGroup {
    /// Id of the subspace inside which the group to remove the user from is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the group from which to remove the user
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// User to be removed from the group
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    /// User signing the message
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgRemoveUserFromUserGroupResponse defines the
/// Msg/RemoveUserFromUserGroupResponse response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgRemoveUserFromUserGroupResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRemoveUserFromUserGroupResponse {}
/// MsgSetUserPermissions represents the message used to set the permissions of a
/// specific user
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgSetUserPermissions")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetUserPermissions {
    /// Id of the subspace inside which to set the permissions
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section for which to set the permissions
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// User for which to set the permissions
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    /// Permissions to be set to the user
    #[prost(string, repeated, tag = "4")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// User signing the message
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSetUserPermissionsResponse defines the Msg/SetPermissionsResponse
/// response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgSetUserPermissionsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetUserPermissionsResponse {}
/// MsgGrantAllowance adds grants for the grantee to spend up allowance of fees
/// from the treasury inside the given subspace
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgGrantAllowance")]
#[serde(rename_all = "snake_case")]
pub struct MsgGrantAllowance {
    /// Id of the subspace inside which where the allowance should be granted
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of the user granting the allowance
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    /// Target being granted the allowance
    #[prost(message, optional, tag = "3")]
    pub grantee: ::core::option::Option<crate::shim::Any>,
    /// Allowance can be any allowance type that implements AllowanceI
    #[prost(message, optional, tag = "4")]
    pub allowance: ::core::option::Option<crate::shim::Any>,
}
/// MsgGrantAllowanceResponse defines the Msg/GrantAllowanceResponse response
/// type.
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgGrantAllowanceResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgGrantAllowanceResponse {}
/// MsgRevokeAllowance removes any existing allowance to the grantee inside the
/// subspace
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgRevokeAllowance")]
#[serde(rename_all = "snake_case")]
pub struct MsgRevokeAllowance {
    /// If of the subspace inside which the allowance to be deleted is
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of the user that created the allowance
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    /// Target being revoked the allowance
    #[prost(message, optional, tag = "3")]
    pub grantee: ::core::option::Option<crate::shim::Any>,
}
/// MsgRevokeAllowanceResponse defines the Msg/RevokeAllowanceResponse
/// response type.
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgRevokeAllowanceResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRevokeAllowanceResponse {}
/// MsgGrantTreasuryAuthorization grants an authorization on behalf of the
/// treasury to a user
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgGrantTreasuryAuthorization")]
#[serde(rename_all = "snake_case")]
pub struct MsgGrantTreasuryAuthorization {
    /// Id of the subspace where the authorization should be granted
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of the user granting a treasury authorization
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    /// Address of the user who is being granted a treasury authorization
    #[prost(string, tag = "3")]
    pub grantee: ::prost::alloc::string::String,
    /// Grant represents the authorization to execute the provided methods
    #[prost(message, optional, tag = "4")]
    pub grant: ::core::option::Option<super::super::super::cosmos::authz::v1beta1::Grant>,
}
/// MsgGrantTreasuryAuthorizationResponse defines the
/// Msg/MsgGrantTreasuryAuthorization response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgGrantTreasuryAuthorizationResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgGrantTreasuryAuthorizationResponse {}
/// MsgRevokeTreasuryAuthorization revokes an existing treasury authorization
/// from a user
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgRevokeTreasuryAuthorization")]
#[serde(rename_all = "snake_case")]
pub struct MsgRevokeTreasuryAuthorization {
    /// Id of the subspace from which the authorization should be revoked
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Address of the user revoking the treasury authorization
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    /// Address of the user who is being revoked the treasury authorization
    #[prost(string, tag = "3")]
    pub grantee: ::prost::alloc::string::String,
    /// Type url of the authorized message which is being revoked
    #[prost(string, tag = "4")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// MsgRevokeTreasuryAuthorizationResponse defines the
/// Msg/MsgRevokeTreasuryAuthorization response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgRevokeTreasuryAuthorizationResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRevokeTreasuryAuthorizationResponse {}
/// MsgUpdateSubspaceFeeTokens represents the message to be used to update the
/// accepted fee tokens inside a given subspace, using an on-chain governance
/// proposal
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgUpdateSubspaceFeeTokens")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateSubspaceFeeTokens {
    /// Id of the subspace where the list of allowed fee tokens will be updated
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// List of tokens allowed to be fee tokens inside the given subspace,
    /// represented as their gas prices
    #[prost(message, repeated, tag = "2")]
    pub additional_fee_tokens:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateSubspaceFeeTokensResponse represents the Msg/UpdateSubspaceFeeTokens
/// response type
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
#[proto_message(type_url = "/desmos.subspaces.v3.MsgUpdateSubspaceFeeTokensResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateSubspaceFeeTokensResponse {}
/// QuerySubspacesRequest is the request type for the Query/Subspaces RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySubspacesRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/Subspaces",
    response_type = QuerySubspacesResponse
)]
pub struct QuerySubspacesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QuerySubspacesResponse is the response type for the Query/Subspaces RPC
/// method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySubspacesResponse")]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspacesResponse {
    #[prost(message, repeated, tag = "1")]
    pub subspaces: ::prost::alloc::vec::Vec<Subspace>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySubspace is the request type for the Query/Subspace RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySubspaceRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/Subspace",
    response_type = QuerySubspaceResponse
)]
pub struct QuerySubspaceRequest {
    /// Id of the subspace to query
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
}
/// QuerySubspaceResponse is the response type for the Query/Subspace method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySubspaceResponse")]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspaceResponse {
    #[prost(message, optional, tag = "1")]
    pub subspace: ::core::option::Option<Subspace>,
}
/// QuerySectionsRequest is the request type for Query/Sections RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySectionsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/Sections",
    response_type = QuerySectionsResponse
)]
pub struct QuerySectionsRequest {
    /// Id of the subspace to query the sections for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QuerySectionsResponse is the response type for Query/Sections RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySectionsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QuerySectionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub sections: ::prost::alloc::vec::Vec<Section>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySectionRequest is the request type for Query/Section RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySectionRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/Section",
    response_type = QuerySectionResponse
)]
pub struct QuerySectionRequest {
    /// Id of the subspace inside which to search for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the searched section
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
}
/// QuerySectionResponse is the response type for Query/Section RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QuerySectionResponse")]
#[serde(rename_all = "snake_case")]
pub struct QuerySectionResponse {
    #[prost(message, optional, tag = "1")]
    pub section: ::core::option::Option<Section>,
}
/// QueryUserGroupsRequest is the request type for the Query/UserGroups RPC
/// method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserGroupsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/UserGroups",
    response_type = QueryUserGroupsResponse
)]
pub struct QueryUserGroupsRequest {
    /// Id of the subspace to query the groups for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// (optional) Section id to query the groups for
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryUserGroupsResponse is the response type for the Query/UserGroups RPC
/// method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserGroupsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<UserGroup>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryUserGroupRequest is the request type for the Query/UserGroup RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserGroupRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/UserGroup",
    response_type = QueryUserGroupResponse
)]
pub struct QueryUserGroupRequest {
    /// Id of the subspace that contains the group
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the group to query
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
}
/// QueryUserGroupResponse is the response type for the Query/UserGroup RPC
/// method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserGroupResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupResponse {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<UserGroup>,
}
/// QueryUserGroupMembersRequest is the request type for the
/// Query/UserGroupMembers RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserGroupMembersRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/UserGroupMembers",
    response_type = QueryUserGroupMembersResponse
)]
pub struct QueryUserGroupMembersRequest {
    /// Id of the subspace that contains the group
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the user group to query the members for
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryUserGroupMembersResponse is the response type for the
/// Query/UserGroupMembers RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserGroupMembersResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupMembersResponse {
    #[prost(string, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryUserPermissionsRequest is the request type for the Query/UserPermissions
/// RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserPermissionsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/UserPermissions",
    response_type = QueryUserPermissionsResponse
)]
pub struct QueryUserPermissionsRequest {
    /// Id of the subspace to query the permissions for
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section to query the permissions for
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// Address of the user to query the permissions for
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
}
/// QueryUserPermissionsRequest is the response type for the
/// Query/UserPermissions method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserPermissionsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryUserPermissionsResponse {
    #[prost(string, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub details: ::prost::alloc::vec::Vec<PermissionDetail>,
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
    desmos_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.subspaces.v3.PermissionDetail")]
#[serde(rename_all = "snake_case")]
pub struct PermissionDetail {
    /// Id of the subspace for which this permission is valid
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// Id of the section for which this permission is valid
    #[prost(uint32, tag = "2")]
    pub section_id: u32,
    /// sum is the oneof that specifies whether this represents a user or
    /// group permission detail
    #[prost(oneof = "permission_detail::Sum", tags = "3, 4")]
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
        desmos_std_derive::CosmwasmExt,
    )]
    #[proto_message(type_url = "/desmos.subspaces.v3.PermissionDetail.User")]
    #[serde(rename_all = "snake_case")]
    pub struct User {
        /// User for which the permission was set
        #[prost(string, tag = "1")]
        pub user: ::prost::alloc::string::String,
        /// Permissions set to the user
        #[prost(string, repeated, tag = "2")]
        pub permission: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
        desmos_std_derive::CosmwasmExt,
    )]
    #[proto_message(type_url = "/desmos.subspaces.v3.PermissionDetail.Group")]
    #[serde(rename_all = "snake_case")]
    pub struct Group {
        /// Unique id of the group
        #[prost(uint32, tag = "1")]
        pub group_id: u32,
        /// Permissions set to the group
        #[prost(string, repeated, tag = "2")]
        pub permission: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
        #[prost(message, tag = "3")]
        User(User),
        /// Group represents a group permission
        #[prost(message, tag = "4")]
        Group(Group),
    }
}
/// QueryUserAllowancesRequest is the request type for the Query/UserAllowances
/// RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserAllowancesRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/UserAllowances",
    response_type = QueryUserAllowancesResponse
)]
pub struct QueryUserAllowancesRequest {
    /// Id of the subspace for which to get the grant(s)
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// (Optional) Address of the user that was granted an allowance
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryUserAllowancesResponse is the response type for the Query/UserAllowances
/// RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryUserAllowancesResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryUserAllowancesResponse {
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGroupAllowancesRequest is the request type for the Query/GroupAllowances
/// RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryGroupAllowancesRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/desmos.subspaces.v3.Query/GroupAllowances",
    response_type = QueryGroupAllowancesResponse
)]
pub struct QueryGroupAllowancesRequest {
    /// Id of the subspace for which to get the grant(s)
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub subspace_id: u64,
    /// (optional) Address of the user group that was granted the allowance(s)
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    /// pagination defines an pagination for the request
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGroupAllowancesResponse is the response type for the
/// Query/GroupAllowances RPC method
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
#[proto_message(type_url = "/desmos.subspaces.v3.QueryGroupAllowancesResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGroupAllowancesResponse {
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
pub struct SubspacesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> SubspacesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn subspaces(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QuerySubspacesResponse, cosmwasm_std::StdError> {
        QuerySubspacesRequest { pagination }.query(self.querier)
    }
    pub fn subspace(
        &self,
        subspace_id: u64,
    ) -> std::result::Result<QuerySubspaceResponse, cosmwasm_std::StdError> {
        QuerySubspaceRequest { subspace_id }.query(self.querier)
    }
    pub fn sections(
        &self,
        subspace_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QuerySectionsResponse, cosmwasm_std::StdError> {
        QuerySectionsRequest {
            subspace_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn section(
        &self,
        subspace_id: u64,
        section_id: u32,
    ) -> std::result::Result<QuerySectionResponse, cosmwasm_std::StdError> {
        QuerySectionRequest {
            subspace_id,
            section_id,
        }
        .query(self.querier)
    }
    pub fn user_groups(
        &self,
        subspace_id: u64,
        section_id: u32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryUserGroupsResponse, cosmwasm_std::StdError> {
        QueryUserGroupsRequest {
            subspace_id,
            section_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn user_group(
        &self,
        subspace_id: u64,
        group_id: u32,
    ) -> std::result::Result<QueryUserGroupResponse, cosmwasm_std::StdError> {
        QueryUserGroupRequest {
            subspace_id,
            group_id,
        }
        .query(self.querier)
    }
    pub fn user_group_members(
        &self,
        subspace_id: u64,
        group_id: u32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryUserGroupMembersResponse, cosmwasm_std::StdError> {
        QueryUserGroupMembersRequest {
            subspace_id,
            group_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn user_permissions(
        &self,
        subspace_id: u64,
        section_id: u32,
        user: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryUserPermissionsResponse, cosmwasm_std::StdError> {
        QueryUserPermissionsRequest {
            subspace_id,
            section_id,
            user,
        }
        .query(self.querier)
    }
    pub fn user_allowances(
        &self,
        subspace_id: u64,
        grantee: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryUserAllowancesResponse, cosmwasm_std::StdError> {
        QueryUserAllowancesRequest {
            subspace_id,
            grantee,
            pagination,
        }
        .query(self.querier)
    }
    pub fn group_allowances(
        &self,
        subspace_id: u64,
        group_id: u32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryGroupAllowancesResponse, cosmwasm_std::StdError> {
        QueryGroupAllowancesRequest {
            subspace_id,
            group_id,
            pagination,
        }
        .query(self.querier)
    }
}
