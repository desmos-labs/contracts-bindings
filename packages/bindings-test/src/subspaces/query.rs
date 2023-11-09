#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, TEST_SUBSPACE_USER_GROUP, USER1_ADDRESS};
    use cosmwasm_std::Coin;
    use desmos_bindings::cosmos_types::{Allowance, BasicAllowance};
    use desmos_bindings::subspaces::types::{
        permission_detail, Grant, Grantee, Permission, PermissionDetail,
    };
    use desmos_bindings::subspaces::types::{
        QueryGroupAllowancesRequest, QueryGroupAllowancesResponse, QuerySubspaceRequest,
        QuerySubspaceResponse, QuerySubspacesRequest, QuerySubspacesResponse,
        QueryUserAllowancesRequest, QueryUserAllowancesResponse, QueryUserGroupMembersRequest,
        QueryUserGroupMembersResponse, QueryUserGroupRequest, QueryUserGroupResponse,
        QueryUserGroupsRequest, QueryUserGroupsResponse, QueryUserPermissionsRequest,
        QueryUserPermissionsResponse,
    };
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_all_subspaces() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QuerySubspacesRequest { pagination: None }.into(),
        };

        let response: QuerySubspacesResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let test_subspace = response.subspaces.first().unwrap();
        assert_eq!("Test subspace", test_subspace.name.as_str());
        assert_eq!("", test_subspace.description.as_str());
        assert_eq!(&contract_address, test_subspace.owner.as_str());
        assert_eq!(&contract_address, test_subspace.creator.as_str());
    }

    #[test]
    fn test_query_subspace() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QuerySubspaceRequest {
                subspace_id: TEST_SUBSPACE,
            }
            .into(),
        };

        let response: QuerySubspaceResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let subspace = response.subspace.unwrap();
        assert_eq!("Test subspace", subspace.name.as_str());
        assert_eq!("", subspace.description.as_str());
        assert_eq!(&contract_address, subspace.owner.as_str());
        assert_eq!(&contract_address, subspace.creator.as_str());
    }

    #[test]
    fn test_query_user_groups() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryUserGroupsRequest {
                subspace_id: TEST_SUBSPACE,
                section_id: 0,
                pagination: None,
            }
            .into(),
        };

        let response: QueryUserGroupsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let user_group = response.groups.get(1).unwrap();
        assert_eq!(TEST_SUBSPACE_USER_GROUP, user_group.id);
        assert_eq!("Test user group", user_group.name.as_str());
        assert_eq!("", user_group.description.as_str());
        assert_eq!(
            vec![Permission::EditSubspace]
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>(),
            user_group.permissions,
        )
    }

    #[test]
    fn test_query_user_group() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryUserGroupRequest {
                subspace_id: TEST_SUBSPACE,
                group_id: TEST_SUBSPACE_USER_GROUP,
            }
            .into(),
        };

        let response: QueryUserGroupResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let test_user_group = response.group.unwrap();
        assert_eq!(TEST_SUBSPACE_USER_GROUP, test_user_group.id);
        assert_eq!("Test user group", test_user_group.name.as_str());
        assert_eq!("", test_user_group.description.as_str());
        assert_eq!(
            vec![Permission::EditSubspace]
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>(),
            test_user_group.permissions
        )
    }

    #[test]
    fn test_query_user_group_members() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryUserGroupMembersRequest {
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
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryUserPermissionsRequest {
                subspace_id: TEST_SUBSPACE,
                section_id: 0,
                user: USER1_ADDRESS.into(),
            }
            .into(),
        };

        let response: QueryUserPermissionsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        assert_eq!(
            vec![
                Permission::EditSubspace,
                Permission::DeleteSubspace,
                Permission::ManageGroups,
            ]
            .into_iter()
            .map(Into::into)
            .collect::<Vec<String>>(),
            response.permissions,
        );
        assert_eq!(
            vec![
                PermissionDetail {
                    subspace_id: 1,
                    section_id: 0,
                    sum: Some(permission_detail::Sum::User(permission_detail::User {
                        user: USER1_ADDRESS.into(),
                        permission: vec![
                            Permission::EditSubspace.into(),
                            Permission::DeleteSubspace.into(),
                            Permission::ManageGroups.into(),
                        ]
                    })),
                },
                PermissionDetail {
                    subspace_id: 1,
                    section_id: 0,
                    sum: Some(permission_detail::Sum::Group(permission_detail::Group {
                        group_id: 0,
                        permission: vec![]
                    })),
                },
                PermissionDetail {
                    subspace_id: 1,
                    section_id: 0,
                    sum: Some(permission_detail::Sum::Group(permission_detail::Group {
                        group_id: 1,
                        permission: vec![Permission::EditSubspace.into()]
                    })),
                }
            ],
            response.details
        );
    }

    #[test]
    fn test_query_user_allowances() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryUserAllowancesRequest {
                subspace_id: TEST_SUBSPACE,
                grantee: USER1_ADDRESS.into(),
                pagination: None,
            }
            .into(),
        };

        let response: QueryUserAllowancesResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let grants = response.grants;

        // Should be only the user group 1 grant
        assert_eq!(1, grants.len());
        assert_eq!(
            &Grant {
                subspace_id: TEST_SUBSPACE,
                granter: contract_address.into(),
                grantee: Some(Grantee::user_grantee(USER1_ADDRESS).into()),
                allowance: Some(
                    Allowance::BasicAllowance(BasicAllowance {
                        spend_limit: vec![Coin::new(1000, "stake").into()],
                        expiration: None,
                    })
                    .into()
                ),
            },
            grants.first().unwrap()
        )
    }

    #[test]
    fn test_query_group_allowances() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        let query = DesmosChain {
            request: QueryGroupAllowancesRequest {
                subspace_id: TEST_SUBSPACE,
                group_id: TEST_SUBSPACE_USER_GROUP,
                pagination: None,
            }
            .into(),
        };

        let response: QueryGroupAllowancesResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let grants = response.grants;

        // Should be only the user1
        assert_eq!(1, grants.len());
        assert_eq!(
            &Grant {
                subspace_id: TEST_SUBSPACE,
                granter: contract_address.into(),
                grantee: Some(Grantee::group_grantee(TEST_SUBSPACE_USER_GROUP).into()),
                allowance: Some(
                    Allowance::BasicAllowance(BasicAllowance {
                        spend_limit: vec![Coin::new(1000, "stake").into()],
                        expiration: None,
                    })
                    .into()
                ),
            },
            grants.first().unwrap()
        )
    }
}
