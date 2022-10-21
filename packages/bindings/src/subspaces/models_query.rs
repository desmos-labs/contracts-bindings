//! Contains the types definitions of all the responses to the x/subspaces query messages.

use crate::subspaces::models::{Permission, Section};
use crate::{
    subspaces::models::{PermissionDetail, Subspace, UserGroup},
    types::PageResponse,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

/// Response to [`Subspaces`](crate::subspaces::query::SubspacesQuery::Subspaces).
#[cw_serde]
pub struct QuerySubspacesResponse {
    /// Queried subspaces.
    pub subspaces: Vec<Subspace>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`Subspace`](crate::subspaces::query::SubspacesQuery::Subspace).
#[cw_serde]
pub struct QuerySubspaceResponse {
    /// Queried subspace.
    pub subspace: Subspace,
}

/// Response to [`Sections`](crate::subspaces::query::SubspacesQuery::Sections).
#[cw_serde]
pub struct QuerySectionsResponse {
    /// Queried sections.
    pub sections: Vec<Section>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`Section`](crate::subspaces::query::SubspacesQuery::Section).
#[cw_serde]
pub struct QuerySectionResponse {
    /// Queried section.
    pub section: Section,
}

/// Response to [`UserGroups`](crate::subspaces::query::SubspacesQuery::UserGroups).
#[cw_serde]
pub struct QueryUserGroupsResponse {
    /// Queried user groups.
    pub groups: Vec<UserGroup>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`UserGroup`](crate::subspaces::query::SubspacesQuery::UserGroup).
#[cw_serde]
pub struct QueryUserGroupResponse {
    /// Queried user group.
    pub group: UserGroup,
}

/// Response to [`UserGroupMembers`](crate::subspaces::query::SubspacesQuery::UserGroupMembers).
#[cw_serde]
pub struct QueryUserGroupMembersResponse {
    /// Members of the queried group.
    pub members: Vec<Addr>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`UserPermissions`](crate::subspaces::query::SubspacesQuery::UserPermissions).
#[cw_serde]
pub struct QueryUserPermissionsResponse {
    /// The user's permissions that is the combination of [details](QueryUserPermissionsResponse::details).
    pub permissions: Vec<Permission>,
    /// List of the user's permissions.
    pub details: Vec<PermissionDetail>,
}
