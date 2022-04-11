//! Contains the query messages that can be sent to the chain in order to query data related
//! to the x/subspaces module.

use crate::types::PageRequest;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Query messages that can be sent to the x/subspaces module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesQuery {
    /// Message to query the created subspaces subspaces.
    /// Response: [QuerySubspacesResponse](crate::subspaces::query_types::QuerySubspacesResponse).
    Subspaces {
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the details of a subspaces.
    /// Response: [QuerySubspaceResponse](crate::subspaces::query_types::QuerySubspaceResponse).
    Subspace {
        /// Subspace id.
        subspace_id: Uint64,
    },
    /// Message to query the groups created inside a sub subspace.
    /// Response: [QueryUserGroupsResponse](crate::subspaces::query_types::QueryUserGroupsResponse).
    UserGroups {
        /// Subspace id to which the groups belongs.
        subspace_id: Uint64,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the details of a group.
    /// Response: [QueryUserGroupResponse](crate::subspaces::query_types::QueryUserGroupResponse).
    UserGroup {
        /// Subspace id to which the group belongs.
        subspace_id: Uint64,
        /// Id of the group.
        group_id: u32,
    },
    /// Message to query the member of a group.
    /// Response: [QueryUserGroupMembersResponse](crate::subspaces::query_types::QueryUserGroupMembersResponse).
    UserGroupMembers {
        /// Subspace id to which the group belongs.
        subspace_id: Uint64,
        /// Id of the group.
        group_id: u32,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Message to query the permission that an user have
    /// inside a subspace.
    /// Response: [QueryUserPermissionsResponse](crate::subspaces::query_types::QueryUserPermissionsResponse).
    UserPermissions {
        /// Subspace id to which the user belongs.
        subspace_id: Uint64,
        /// User address.
        user: Addr,
    },
}
