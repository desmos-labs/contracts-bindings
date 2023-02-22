#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, USER1_ADDRESS};
    use cosmwasm_std::Addr;
    use desmos_bindings::stargate::relationships::msg::RelationshipsMsgBuilder;
    use test_contract::msg::ExecuteMsg::DesmosMessages;

    #[test]
    fn test_create_delete_relationship() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_relationship = RelationshipsMsgBuilder::create_relationship(
            Addr::unchecked(&contract_address),
            Addr::unchecked(USER1_ADDRESS),
            TEST_SUBSPACE.into(),
        );

        let msg = DesmosMessages {
            msgs: vec![create_relationship.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let delete_relationship = RelationshipsMsgBuilder::delete_relationship(
            Addr::unchecked(&contract_address),
            Addr::unchecked(USER1_ADDRESS),
            TEST_SUBSPACE.into(),
        );

        let msg = DesmosMessages {
            msgs: vec![delete_relationship.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_block_unblock_user() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let block_user = RelationshipsMsgBuilder::block_user(
            Addr::unchecked(&contract_address),
            Addr::unchecked(USER1_ADDRESS),
            "".into(),
            TEST_SUBSPACE.into(),
        );

        let msg = DesmosMessages {
            msgs: vec![block_user.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let unblock_user = RelationshipsMsgBuilder::unblock_user(
            Addr::unchecked(&contract_address),
            Addr::unchecked(USER1_ADDRESS),
            TEST_SUBSPACE.into(),
        );

        let msg = DesmosMessages {
            msgs: vec![unblock_user.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
