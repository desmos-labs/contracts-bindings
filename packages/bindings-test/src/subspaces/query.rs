#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, TEST_SUBSPACE_USER_GROUP, USER1_ADDRESS};
    use cosmwasm_std::{Addr, Uint64};
    use desmos_bindings::subspaces::models::{
        GroupPermissions, Permission, PermissionDetail, UserPermissions,
    };
    use desmos_bindings::subspaces::query::SubspacesQuery;
    use desmos_bindings::subspaces::query_types::{
        QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
        QueryUserGroupResponse, QueryUserGroupsResponse, QueryUserPermissionsResponse,
    };
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_all_subspaces() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: SubspacesQuery::Subspaces { pagination: None }.into(),
        };

        let response: QuerySubspacesResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let test_subspace = response.subspaces.first().unwrap();
        assert_eq!("Test subspace", test_subspace.name.as_str());
        assert_eq!("", test_subspace.description.as_str());
        assert_eq!(&contract_address, test_subspace.treasury.as_str());
        assert_eq!(&contract_address, test_subspace.owner.as_str());
        assert_eq!(&contract_address, test_subspace.creator.as_str());
    }

    #[test]
    fn test_query_test_subspace() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: SubspacesQuery::Subspace {
                subspace_id: TEST_SUBSPACE,
            }
            .into(),
        };

        let response: QuerySubspaceResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let test_subspace = response.subspace;
        assert_eq!("Test subspace", test_subspace.name.as_str());
        assert_eq!("", test_subspace.description.as_str());
        assert_eq!(&contract_address, test_subspace.treasury.as_str());
        assert_eq!(&contract_address, test_subspace.owner.as_str());
        assert_eq!(&contract_address, test_subspace.creator.as_str());
    }

    #[test]
    fn test_query_user_groups() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: SubspacesQuery::UserGroups {
                subspace_id: TEST_SUBSPACE,
                section_id: None,
                pagination: None,
            }
            .into(),
        };

        let response: QueryUserGroupsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let test_user_group = response.groups.get(1).unwrap();
        assert_eq!(TEST_SUBSPACE_USER_GROUP, test_user_group.id);
        assert_eq!("Test user group", test_user_group.name.as_str());
        assert_eq!("", test_user_group.description.as_str());
        assert_eq!(vec![Permission::Everything], test_user_group.permissions)
    }

    #[test]
    fn test_query_user_group() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: SubspacesQuery::UserGroup {
                subspace_id: TEST_SUBSPACE,
                group_id: TEST_SUBSPACE_USER_GROUP,
            }
            .into(),
        };

        let response: QueryUserGroupResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let test_user_group = response.group;
        assert_eq!(TEST_SUBSPACE_USER_GROUP, test_user_group.id);
        assert_eq!("Test user group", test_user_group.name.as_str());
        assert_eq!("", test_user_group.description.as_str());
        assert_eq!(vec![Permission::Everything], test_user_group.permissions)
    }

    #[test]
    fn test_query_user_group_members() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: SubspacesQuery::UserGroupMembers {
                subspace_id: TEST_SUBSPACE,
                group_id: TEST_SUBSPACE_USER_GROUP,
                pagination: None,
            }
            .into(),
        };

        let response: QueryUserGroupMembersResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let members = response.members;

        // Should be only the user1
        assert_eq!(1, members.len());
        assert_eq!(USER1_ADDRESS, members.first().unwrap().as_str())
    }

    #[test]
    fn test_query_user_permissions() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let query = DesmosChain {
            request: SubspacesQuery::UserPermissions {
                subspace_id: TEST_SUBSPACE,
                section_id: None,
                user: Addr::unchecked(USER1_ADDRESS),
            }
            .into(),
        };

        let response: QueryUserPermissionsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        assert_eq!(vec![Permission::Everything], response.permissions);
        assert_eq!(
            vec![
                PermissionDetail {
                    subspace_id: Uint64::new(1),
                    section_id: 0,
                    user: Some(UserPermissions {
                        user: Addr::unchecked(USER1_ADDRESS),
                        permission: vec![
                            Permission::EditSubspace,
                            Permission::DeleteSubspace,
                            Permission::ManageGroups
                        ]
                    }),
                    group: None
                },
                PermissionDetail {
                    subspace_id: Uint64::new(1),
                    section_id: 0,
                    user: None,
                    group: Some(GroupPermissions {
                        group_id: 0,
                        permission: vec![]
                    })
                },
                PermissionDetail {
                    subspace_id: Uint64::new(1),
                    section_id: 0,
                    user: None,
                    group: Some(GroupPermissions {
                        group_id: 1,
                        permission: vec![Permission::Everything]
                    })
                }
            ],
            response.details
        );
    }
}
