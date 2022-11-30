//! Contains structs and enums related to the x/subspaces module.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

/// Struct that represents a subspace.
#[cw_serde]
pub struct Subspace {
    /// Unique id of the subspace.
    pub id: Uint64,
    /// Subspace name.
    pub name: String,
    /// Subspace description.
    pub description: String,
    /// Subspace treasury.
    pub treasury: Addr,
    /// Subspace owner.
    pub owner: Addr,
    /// Subspace creator.
    pub creator: Addr,
    /// Subspace creation time in RFC 3339 format.
    /// example: 1972-01-01T10:00:20.
    pub creation_time: String,
}

/// Contains the data of a single subspace section.
#[cw_serde]
pub struct Section {
    /// Id of the subspace inside which the section exists.
    pub subspace_id: Uint64,
    /// Unique id of the section within the subspace.
    pub id: u32,
    /// Id of the parent section.
    pub parent_id: Option<u32>,
    /// Name of the section within the subspace.
    pub name: String,
    /// Description of the section.
    pub description: String,
}

/// Structs that represents a user group.
#[cw_serde]
pub struct UserGroup {
    /// Subspace to which the group belongs.
    pub subspace_id: Uint64,
    /// Id of the section inside which this group is valid.
    pub section_id: Option<u32>,
    /// User group id.
    pub id: u32,
    /// Group name.
    pub name: String,
    /// Group description.
    pub description: String,
    /// Permissions that all the group members inherit.
    pub permissions: Vec<Permission>,
}

/// Enum that represents a permission that has been given to an user.
#[cw_serde]
pub struct PermissionDetail {
    /// Id of the subspace for which this permission is valid.
    pub subspace_id: Uint64,
    /// Id of the section for which this permission is valid.
    pub section_id: u32,
    /// User permissions.
    pub user: Option<UserPermissions>,
    /// Group permissions.
    pub group: Option<GroupPermissions>,
}

/// Struct that represent the permissions given to an user.
#[cw_serde]
pub struct UserPermissions {
    /// User for which the permission was set.
    pub user: Addr,
    /// Permissions set to the user.
    pub permission: Vec<Permission>,
}

/// Struct that represents a permissions given to a group.
#[cw_serde]
pub struct GroupPermissions {
    /// Group for which the permission was set.
    pub group_id: u32,
    /// Permissions set to the group.
    pub permission: Vec<Permission>,
}

/// Represents the permissions that can be given to an user or a user group.
#[cw_serde]
pub enum Permission {
    /// Allows to change the information of the subspace.
    #[serde(rename = "EDIT_SUBSPACE")]
    EditSubspace,
    /// Allows users to delete the subspace.
    #[serde(rename = "DELETE_SUBSPACE")]
    DeleteSubspace,
    /// Allows users to manage a subspace sections.
    #[serde(rename = "MANAGE_SECTIONS")]
    ManageSections,
    /// Allows users to manage user groups and members.
    #[serde(rename = "MANAGE_GROUPS")]
    ManageGroups,
    /// Allows users to set other users' permissions (except [`Permission::`SetPermissions`]).
    /// This includes managing user groups and the associated permissions.
    #[serde(rename = "SET_PERMISSIONS")]
    SetPermissions,
    /// Allows to do everything.
    /// This should usually be reserved only to the owner (which has it by default).
    #[serde(rename = "EVERYTHING")]
    Everything,
    /// Identifies users that can create content inside the subspace.
    #[serde(rename = "WRITE_CONTENT")]
    Write,
    /// Allows users to interact with content inside the subspace (eg. polls).
    #[serde(rename = "INTERACT_WITH_CONTENT")]
    InteractWithContent,
    /// Allows users to edit their own content inside the subspace.
    #[serde(rename = "EDIT_OWN_CONTENT")]
    EditOwnContent,
    /// Allows users to moderate other user's content.
    #[serde(rename = "MODERATE_CONTENT")]
    ModerateContent,
}
