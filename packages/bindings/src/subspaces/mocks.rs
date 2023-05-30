//! Contains useful mocks of the Desmos x/subspaces module's types made to be used in any test.

use crate::subspaces::types::Permission;
use crate::subspaces::types::{
    permission_detail::{Sum, User},
    PermissionDetail, QuerySectionResponse, QuerySectionsResponse, QuerySubspaceResponse,
    QuerySubspacesResponse, QueryUserGroupMembersResponse, QueryUserGroupResponse,
    QueryUserGroupsResponse, QueryUserPermissionsResponse, Section, Subspace, UserGroup,
};
use crate::types::Timestamp;

use chrono::DateTime;

/// Represents the mock subspace owner for unit test.
pub const MOCK_SUBSPACE_OWNER: &str = "owner";

/// Represents the mock subspace treasury for unit test.
pub const MOCK_SUBSPACE_TREASURY: &str = "treasury";

/// Represents the mock subspace creator for unit test.
pub const MOCK_SUBSPACE_CREATOR: &str = "creator";

/// Represents the mock permissioned user in the subspace for unit test.
pub const MOCK_PERMISSIONED_USER: &str = "permissioned_user";

/// Struct that contains some utility methods to mock data of the Desmos
/// x/subspaces module.
pub struct MockSubspacesQueries {}

impl MockSubspacesQueries {
    /// Gets a mocked instance of [`Subspace`].
    pub fn get_mocked_subspace(id: u64) -> Subspace {
        Subspace {
            id,
            name: "Test subspace".into(),
            description: "Test subspace".into(),
            treasury: MOCK_SUBSPACE_OWNER.into(),
            owner: MOCK_SUBSPACE_TREASURY.into(),
            creator: MOCK_SUBSPACE_CREATOR.into(),
            creation_time: Some(Timestamp::from(DateTime::from(
                DateTime::parse_from_rfc3339("2022-02-21T13:18:57.800827Z").unwrap(),
            ))),
        }
    }

    /// Gets a mocked instance of [`Section`].
    pub fn get_mocked_section(subspace_id: u64, id: u32) -> Section {
        Section {
            subspace_id,
            id,
            parent_id: 0,
            name: "Section name".to_string(),
            description: "Section description".to_string(),
        }
    }

    /// Get a mocked instance of [`UserGroup`].
    pub fn get_mocked_user_group(subspace_id: u64, section_id: u32, id: u32) -> UserGroup {
        UserGroup {
            id,
            section_id,
            subspace_id,
            name: String::from("Test group"),
            description: String::from("Test group"),
            permissions: vec![Permission::EditSubspace.into()],
        }
    }

    /// Gets a mocked instance of [`PermissionDetail`].
    pub fn get_mocked_permission_detail(subspace_id: u64, section_id: u32) -> PermissionDetail {
        PermissionDetail {
            subspace_id,
            section_id,
            sum: Some(Sum::User(User {
                user: MOCK_PERMISSIONED_USER.into(),
                permission: vec![Permission::EditSubspace.into()],
            })),
        }
    }

    /// Gets a mocked instance of [`QuerySubspacesResponse`].
    pub fn get_mocked_subspaces_response() -> QuerySubspacesResponse {
        QuerySubspacesResponse {
            subspaces: vec![Self::get_mocked_subspace(1)],
            pagination: None,
        }
    }

    /// Gets a mocked instance of [`QuerySubspaceResponse`].
    pub fn get_mocked_subspace_response() -> QuerySubspaceResponse {
        QuerySubspaceResponse {
            subspace: Some(Self::get_mocked_subspace(1)),
        }
    }

    /// Gets a mocked instance of [`QuerySectionsResponse`].
    pub fn get_mocked_sections_response() -> QuerySectionsResponse {
        QuerySectionsResponse {
            sections: vec![Self::get_mocked_section(1, 1)],
            pagination: None,
        }
    }

    /// Gets a mocked instance of [`QuerySectionResponse`].
    pub fn get_mocked_section_response() -> QuerySectionResponse {
        QuerySectionResponse {
            section: Some(Self::get_mocked_section(1, 1)),
        }
    }

    /// Gets a mocked instance of [`QueryUserGroupsResponse`].
    pub fn get_mocked_user_groups_response() -> QueryUserGroupsResponse {
        QueryUserGroupsResponse {
            groups: vec![Self::get_mocked_user_group(1, 0, 1)],
            pagination: None,
        }
    }

    /// Gets a mocked instance of [`QueryUserGroupResponse`].
    pub fn get_mocked_user_group_response() -> QueryUserGroupResponse {
        QueryUserGroupResponse {
            group: Some(Self::get_mocked_user_group(1, 0, 1)),
        }
    }

    /// Gets a mocked instance of [`QueryUserGroupMembersResponse`].
    pub fn get_mocked_user_group_members_response() -> QueryUserGroupMembersResponse {
        QueryUserGroupMembersResponse {
            members: vec![MOCK_PERMISSIONED_USER.into()],
            pagination: None,
        }
    }

    /// Gets a mocked instance of [`QueryUserPermissionsResponse`].
    pub fn get_mocked_user_permissions_response() -> QueryUserPermissionsResponse {
        QueryUserPermissionsResponse {
            permissions: vec![Permission::EditSubspace.into()],
            details: vec![Self::get_mocked_permission_detail(1, 0)],
        }
    }
}
