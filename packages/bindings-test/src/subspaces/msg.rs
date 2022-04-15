#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use cosmwasm_std::{Addr, Uint64};
    use desmos_bindings::subspaces::msg::SubspacesMsg;
    use test_contract::msg::ExecuteMsg::DesmosMessages;

    #[test]
    fn test_subspace_create_delete() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_subspace = SubspacesMsg::CreateSubspace {
            creator: Addr::unchecked(&contract_address),
            name: "test_subspace_create_delete".to_string(),
            description: "".to_string(),
            owner: Addr::unchecked(&contract_address),
            treasury: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![create_subspace.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();

        let delete_subspace = SubspacesMsg::DeleteSubspace {
            subspace_id: Uint64::new(2),
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![delete_subspace.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
