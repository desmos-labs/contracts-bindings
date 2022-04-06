//! Contains structs and enums related to the x/subspaces module.

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Struct that represents a subspace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    /// Subspace creation time.
    pub creation_time: String,
}

/// Structs that represents a user group.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserGroup {
    /// Subspace to which the group belong.
    pub subspace_id: Uint64,
    /// User group id.
    pub id: u32,
    /// Group name.
    pub name: String,
    /// Group description.
    pub description: String,
    /// Permissions that all the group members inherit.
    pub permissions: u32,
}

/// Enum that represents a permission that has been given to an user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionDetail {
    /// Permission directly given to an user.
    User(UserPermission),
    /// Permission that the user had inherit from a group.
    Group(GroupPermission),
}

/// Struct that represent a permission given to an user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserPermission {
    /// User for which the permission was set.
    pub user: String,
    /// Permissions set to the user.
    pub permissions: u32,
}

/// Struct that represents a permission given to a group.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GroupPermission {
    /// Group for which the permission was set.
    pub group_id: u32,
    /// Permission set to the group.
    pub permissions: u32,
}
