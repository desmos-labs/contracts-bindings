#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, TEST_SUBSPACE_USER_GROUP, USER2_ADDRESS};
    use cosmwasm_std::{Addr, Uint64};
    use desmos_bindings::legacy::subspaces::models::Permission;
    use desmos_bindings::legacy::subspaces::msg::SubspacesMsg;
    use test_contract::msg::ExecuteMsg;
    use test_contract::msg::ExecuteMsg::DesmosMessages;

    fn build_create_subspace_msg(contract_address: &str) -> ExecuteMsg {
        DesmosMessages {
            msgs: vec![SubspacesMsg::CreateSubspace {
                creator: Addr::unchecked(contract_address),
                name: "test_subspace_create_delete".to_string(),
                description: "".to_string(),
                owner: Addr::unchecked(contract_address),
                treasury: Addr::unchecked(contract_address),
            }
            .into()],
        }
    }

    #[test]
    fn test_create_subspace() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_subspace_msg = build_create_subspace_msg(&contract_address);

        desmos_cli
            .wasm_execute(&contract_address, &create_subspace_msg)
            .assert_success();
    }

    #[test]
    fn test_edit_subspace() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subspace_id = TEST_SUBSPACE;
        let new_subspace_name = "Test subspace";

        let edit_subspace = SubspacesMsg::EditSubspace {
            subspace_id,
            name: new_subspace_name.to_string(),
            description: "".to_string(),
            treasury: Addr::unchecked(&contract_address),
            owner: Addr::unchecked(&contract_address),
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![edit_subspace.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let subspace = desmos_cli.query_subspace(subspace_id).subspace;
        assert_eq!(new_subspace_name, subspace.name);
    }

    #[test]
    fn test_delete_subspace() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_subspace_msg = build_create_subspace_msg(&contract_address);

        desmos_cli
            .wasm_execute(&contract_address, &create_subspace_msg)
            .assert_success();

        // Get the id of the last created subspace.
        let response = desmos_cli.query_subspaces(None);
        let subspace_id = response.subspaces.last().unwrap().id;

        // Delete the previously created subspace.
        let delete_subspace = SubspacesMsg::DeleteSubspace {
            subspace_id,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![delete_subspace.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    fn build_create_user_group_msg(subspace_id: Uint64, contract_address: &str) -> ExecuteMsg {
        DesmosMessages {
            msgs: vec![SubspacesMsg::CreateUserGroup {
                subspace_id,
                section_id: None,
                name: "test_user_group".to_string(),
                description: None,
                initial_members: vec![],
                default_permissions: vec![Permission::EditSubspace],
                creator: Addr::unchecked(contract_address),
            }
            .into()],
        }
    }

    #[test]
    pub fn test_create_user_group() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_user_group_msg = build_create_user_group_msg(TEST_SUBSPACE, &contract_address);

        desmos_cli
            .wasm_execute(&contract_address, &create_user_group_msg)
            .assert_success();
    }

    #[test]
    pub fn test_delete_user_group() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subspace_id = TEST_SUBSPACE;

        // Create the user group to delete.
        let create_user_group_msg = build_create_user_group_msg(TEST_SUBSPACE, &contract_address);

        desmos_cli
            .wasm_execute(&contract_address, &create_user_group_msg)
            .assert_success();

        // Get the id of the created user group.
        let response = desmos_cli.query_user_groups(subspace_id, None);
        let group_id = response.groups.last().unwrap().id;

        // Delete the created user group.
        let delete_user_group = SubspacesMsg::DeleteUserGroup {
            subspace_id,
            group_id,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![delete_user_group.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    pub fn test_edit_user_group() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subspace_id = TEST_SUBSPACE;
        let group_id = TEST_SUBSPACE_USER_GROUP;
        let new_user_group_name = "Test user group";

        let edit_user_group = SubspacesMsg::EditUserGroup {
            subspace_id,
            group_id,
            name: Some(new_user_group_name.to_string()),
            description: None,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![edit_user_group.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let response = desmos_cli.query_user_group(subspace_id, group_id);
        assert_eq!(new_user_group_name, response.group.name);
        assert!(response.group.description.is_empty());
    }

    #[test]
    pub fn test_set_user_group_permissions() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subspace_id = TEST_SUBSPACE;
        let group_id = TEST_SUBSPACE_USER_GROUP;
        let new_permissions = vec![Permission::EditSubspace];

        let set_user_group_permissions = SubspacesMsg::SetUserGroupPermissions {
            subspace_id,
            group_id,
            permissions: new_permissions.clone(),
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![set_user_group_permissions.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let response = desmos_cli.query_user_group(subspace_id, group_id);
        assert_eq!(&new_permissions, &response.group.permissions);
    }

    #[test]
    pub fn test_add_remove_user_from_user_group() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subspace_id = TEST_SUBSPACE;
        let group_id = TEST_SUBSPACE_USER_GROUP;

        let add_user_to_group = SubspacesMsg::AddUserToUserGroup {
            subspace_id,
            group_id,
            user: Addr::unchecked(USER2_ADDRESS),
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![add_user_to_group.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let response = desmos_cli.query_user_group_members(subspace_id, group_id, None);

        assert!(response.members.contains(&Addr::unchecked(USER2_ADDRESS)));

        let remove_user_from_group = SubspacesMsg::RemoveUserFromUserGroup {
            subspace_id,
            group_id,
            user: Addr::unchecked(USER2_ADDRESS),
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![remove_user_from_group.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    pub fn test_set_user_permissions() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subspace_id = TEST_SUBSPACE;
        let new_permissions = vec![
            Permission::EditSubspace,
            Permission::DeleteSubspace,
            Permission::ManageGroups,
            Permission::Write,
            Permission::InteractWithContent,
            Permission::EditOwnContent,
            Permission::ModerateContent,
        ];

        let set_user_permissions = SubspacesMsg::SetUserPermissions {
            subspace_id,
            section_id: 0,
            user: Addr::unchecked(USER2_ADDRESS),
            permissions: new_permissions,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![set_user_permissions.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
