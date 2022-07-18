//! Contains some useful mocks of the Desmos x/subspaces module's types made to be used in any test.

use crate::subspaces::models::{Permission, Section, UserPermissions};
use crate::subspaces::models_query::{QuerySectionResponse, QuerySectionsResponse};
use crate::subspaces::{
    models::{PermissionDetail, Subspace, UserGroup},
    models_query::{
        QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
        QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
    },
    query::SubspacesQuery,
};
use cosmwasm_std::{to_binary, Addr, Binary, ContractResult, Uint64};

/// Struct that contains some utility methods to mock data of the Desmos
/// x/subspaces module.
pub struct MockSubspacesQueries;

impl MockSubspacesQueries {
    /// Gets a mocked instance of [`Subspace`].
    pub fn get_mock_subspace() -> Subspace {
        Subspace {
            id: Uint64::new(1),
            name: "Test subspace".to_string(),
            description: "Test subspace".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            creation_time: "2022-02-21T13:18:57.800827Z".to_string(),
        }
    }

    /// Gets a mocked instance of [`Section`].
    pub fn get_mock_section() -> Section {
        Section {
            subspace_id: Uint64::new(1),
            id: 1,
            parent_id: None,
            name: "Section name".to_string(),
            description: "Section description".to_string(),
        }
    }

    /// Get a mocked instance of [`UserGroup`].
    pub fn get_mock_user_group() -> UserGroup {
        UserGroup {
            id: 1,
            section_id: None,
            subspace_id: Uint64::new(1),
            name: String::from("Test group"),
            description: String::from("Test group"),
            permissions: vec![Permission::EditSubspace],
        }
    }

    /// Gets a mocked user group member.
    pub fn get_mock_group_member() -> Addr {
        Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69")
    }

    /// Gets a mocked instance of [`PermissionDetail`].
    pub fn get_mock_permission_detail() -> PermissionDetail {
        PermissionDetail {
            subspace_id: Uint64::new(1),
            section_id: 0,
            user: Some(UserPermissions {
                user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
                permission: vec![Permission::EditSubspace],
            }),
            group: None,
        }
    }
}

/// Functions that mocks the subspaces query responses.
pub fn mock_subspaces_query_response(query: &SubspacesQuery) -> ContractResult<Binary> {
    let response = match query {
        SubspacesQuery::Subspaces { .. } => {
            let subspace = MockSubspacesQueries::get_mock_subspace();
            to_binary(&QuerySubspacesResponse {
                subspaces: vec![subspace],
                pagination: Default::default(),
            })
        }
        SubspacesQuery::Subspace { .. } => {
            let subspace = MockSubspacesQueries::get_mock_subspace();
            to_binary(&QuerySubspaceResponse { subspace })
        }
        SubspacesQuery::Sections { .. } => to_binary(&QuerySectionsResponse {
            sections: vec![MockSubspacesQueries::get_mock_section()],
            pagination: None,
        }),
        SubspacesQuery::Section { .. } => to_binary(&QuerySectionResponse {
            section: MockSubspacesQueries::get_mock_section(),
        }),
        SubspacesQuery::UserGroups { .. } => {
            let group = MockSubspacesQueries::get_mock_user_group();
            to_binary(&QueryUserGroupsResponse {
                groups: vec![group],
                pagination: Default::default(),
            })
        }
        SubspacesQuery::UserGroup { .. } => {
            let group = MockSubspacesQueries::get_mock_user_group();
            to_binary(&QueryUserGroupResponse { group })
        }
        SubspacesQuery::UserGroupMembers { .. } => {
            let member = MockSubspacesQueries::get_mock_group_member();
            to_binary(&QueryUserGroupMembersResponse {
                members: vec![member],
                pagination: Default::default(),
            })
        }
        SubspacesQuery::UserPermissions { .. } => {
            let permission = MockSubspacesQueries::get_mock_permission_detail();
            to_binary(&QueryUserPermissionsResponse {
                permissions: Default::default(),
                details: vec![permission],
            })
        }
    };
    response.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_subspaces() {
        let query = SubspacesQuery::Subspaces {
            pagination: Default::default(),
        };
        let response = mock_subspaces_query_response(&query);
        let expected = to_binary(&QuerySubspacesResponse {
            subspaces: vec![MockSubspacesQueries::get_mock_subspace()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_subspace() {
        let query = SubspacesQuery::Subspace {
            subspace_id: Uint64::new(1),
        };
        let response = mock_subspaces_query_response(&query);
        let expected = to_binary(&QuerySubspaceResponse {
            subspace: MockSubspacesQueries::get_mock_subspace(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_groups() {
        let query = SubspacesQuery::UserGroups {
            subspace_id: Uint64::new(1),
            section_id: None,
            pagination: Default::default(),
        };
        let response = mock_subspaces_query_response(&query);
        let expected = to_binary(&QueryUserGroupsResponse {
            groups: vec![MockSubspacesQueries::get_mock_user_group()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_group() {
        let query = SubspacesQuery::UserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
        };
        let response = mock_subspaces_query_response(&query);
        let expected = to_binary(&QueryUserGroupResponse {
            group: MockSubspacesQueries::get_mock_user_group(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_group_members() {
        let query = SubspacesQuery::UserGroupMembers {
            subspace_id: Uint64::new(1),
            group_id: 1,
            pagination: Default::default(),
        };
        let response = mock_subspaces_query_response(&query);
        let expected = to_binary(&QueryUserGroupMembersResponse {
            members: vec![MockSubspacesQueries::get_mock_group_member()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }

    #[test]
    fn test_query_user_permissions() {
        let query = SubspacesQuery::UserPermissions {
            subspace_id: Uint64::new(1),
            section_id: None,
            user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        let response = mock_subspaces_query_response(&query);
        let expected = to_binary(&QueryUserPermissionsResponse {
            permissions: Default::default(),
            details: vec![MockSubspacesQueries::get_mock_permission_detail()],
        });
        assert_eq!(response.into_result().ok(), expected.ok());
    }
}
