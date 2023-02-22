#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, TEST_SUBSPACE_USER_GROUP, USER2_ADDRESS};
    use cosmwasm_std::{Addr, Uint64};
    use desmos_bindings::stargate::subspaces::msg::SubspacesMsgBuilder;
    use desmos_bindings::stargate::subspaces::types::Permission;
    use test_contract::msg::ExecuteMsg;
    use test_contract::msg::ExecuteMsg::DesmosMessages;

    fn build_create_subspace_msg(contract_address: &str) -> ExecuteMsg {
        DesmosMessages {
            msgs: vec![SubspacesMsgBuilder::create_subspace(
                "test_subspace_create_delete",
                "",
                Addr::unchecked(contract_address),
                Addr::unchecked(contract_address),
            )
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

        let edit_subspace = SubspacesMsgBuilder::edit_subspace(
            subspace_id.into(),
            new_subspace_name,
            "",
            Addr::unchecked(&contract_address),
            Addr::unchecked(&contract_address),
        );

        let msg = DesmosMessages {
            msgs: vec![edit_subspace.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
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
        let delete_subspace = SubspacesMsgBuilder::delete_subspace(
            subspace_id.into(),
            Addr::unchecked(&contract_address),
        );

        let msg = DesmosMessages {
            msgs: vec![delete_subspace.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    fn build_create_user_group_msg(subspace_id: Uint64, contract_address: &str) -> ExecuteMsg {
        DesmosMessages {
            msgs: vec![SubspacesMsgBuilder::create_user_group(
                subspace_id.into(),
                None,
                "test_user_group",
                None,
                vec![Permission::EditSubspace],
                vec![],
                Addr::unchecked(contract_address),
            )
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
        let delete_user_group = SubspacesMsgBuilder::delete_user_group(
            subspace_id.into(),
            group_id,
            Addr::unchecked(&contract_address),
        );

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

        let edit_user_group = SubspacesMsgBuilder::edit_user_group(
            subspace_id.into(),
            group_id,
            Some(new_user_group_name),
            None,
            Addr::unchecked(&contract_address),
        );

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

        let set_user_group_permissions = SubspacesMsgBuilder::set_user_group_permissions(
            subspace_id.into(),
            group_id,
            new_permissions,
            Addr::unchecked(&contract_address),
        );

        let msg = DesmosMessages {
            msgs: vec![set_user_group_permissions.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    pub fn test_add_remove_user_from_user_group() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);
        let subspace_id = TEST_SUBSPACE;
        let group_id = TEST_SUBSPACE_USER_GROUP;

        let add_user_to_group = SubspacesMsgBuilder::add_user_to_user_group(
            subspace_id.into(),
            group_id,
            Addr::unchecked(USER2_ADDRESS),
            Addr::unchecked(&contract_address),
        );

        let msg = DesmosMessages {
            msgs: vec![add_user_to_group.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let response = desmos_cli.query_user_group_members(subspace_id, group_id, None);

        assert!(response.members.contains(&Addr::unchecked(USER2_ADDRESS)));

        let remove_user_from_group = SubspacesMsgBuilder::remove_user_from_user_group(
            subspace_id.into(),
            group_id,
            Addr::unchecked(USER2_ADDRESS),
            Addr::unchecked(&contract_address),
        );

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

        let set_user_permissions = SubspacesMsgBuilder::set_user_permissions(
            subspace_id.into(),
            0,
            Addr::unchecked(USER2_ADDRESS),
            new_permissions,
            Addr::unchecked(&contract_address),
        );

        let msg = DesmosMessages {
            msgs: vec![set_user_permissions.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
