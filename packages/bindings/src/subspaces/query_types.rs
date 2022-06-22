//! Contains the types definitions of all the responses to the x/subspaces query messages.

use crate::subspaces::models::{Permission, Section};
use crate::{
    subspaces::models::{PermissionDetail, Subspace, UserGroup},
    types::PageResponse,
};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Response to [`Subspaces`](crate::subspaces::query::SubspacesQuery::Subspaces).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspacesResponse {
    /// Queried subspaces.
    pub subspaces: Vec<Subspace>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`Subspace`](crate::subspaces::query::SubspacesQuery::Subspace).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySubspaceResponse {
    /// Queried subspace.
    pub subspace: Subspace,
}

/// Response to [`Sections`](crate::subspaces::query::SubspacesQuery::Sections).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySectionsResponse {
    /// Queried sections.
    pub sections: Vec<Section>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`Section`](crate::subspaces::query::SubspacesQuery::Section).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuerySectionResponse {
    /// Queried section.
    pub section: Section,
}

/// Response to [`UserGroups`](crate::subspaces::query::SubspacesQuery::UserGroups).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupsResponse {
    /// Queried user groups.
    pub groups: Vec<UserGroup>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`UserGroup`](crate::subspaces::query::SubspacesQuery::UserGroup).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupResponse {
    /// Queried user group.
    pub group: UserGroup,
}

/// Response to [`UserGroupMembers`](crate::subspaces::query::SubspacesQuery::UserGroupMembers).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserGroupMembersResponse {
    /// Members of the queried group.
    pub members: Vec<Addr>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`UserPermissions`](crate::subspaces::query::SubspacesQuery::UserPermissions).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserPermissionsResponse {
    /// The user's permissions that is the combination of [details](QueryUserPermissionsResponse::details).
    pub permissions: Vec<Permission>,
    /// List of the user's permissions.
    pub details: Vec<PermissionDetail>,
}
