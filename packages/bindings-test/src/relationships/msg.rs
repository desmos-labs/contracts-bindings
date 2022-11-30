#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{TEST_SUBSPACE, USER1_ADDRESS};
    use cosmwasm_std::Addr;
    use desmos_bindings::legacy::relationships::msg::RelationshipsMsg;
    use test_contract::msg::ExecuteMsg::DesmosMessages;

    #[test]
    fn test_create_delete_relationship() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_relationship = RelationshipsMsg::CreateRelationship {
            signer: Addr::unchecked(&contract_address),
            counterparty: Addr::unchecked(USER1_ADDRESS),
            subspace_id: TEST_SUBSPACE,
        };

        let msg = DesmosMessages {
            msgs: vec![create_relationship.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let delete_relationship = RelationshipsMsg::DeleteRelationship {
            signer: Addr::unchecked(&contract_address),
            counterparty: Addr::unchecked(USER1_ADDRESS),
            subspace_id: TEST_SUBSPACE,
        };

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

        let block_user = RelationshipsMsg::BlockUser {
            blocker: Addr::unchecked(&contract_address),
            blocked: Addr::unchecked(USER1_ADDRESS),
            reason: "".to_string(),
            subspace_id: TEST_SUBSPACE,
        };

        let msg = DesmosMessages {
            msgs: vec![block_user.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let unblock_user = RelationshipsMsg::UnblockUser {
            blocker: Addr::unchecked(&contract_address),
            blocked: Addr::unchecked(USER1_ADDRESS),
            subspace_id: TEST_SUBSPACE,
        };

        let msg = DesmosMessages {
            msgs: vec![unblock_user.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
