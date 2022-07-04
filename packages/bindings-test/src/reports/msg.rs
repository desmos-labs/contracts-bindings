#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_DELETABLE_REASON_ID, TEST_DELETABLE_REPORT_ID, TEST_REASON_ID, TEST_SUBSPACE,
        USER1_ADDRESS,
    };
    use cosmwasm_std::Addr;
    use desmos_bindings::reports::models::ReportTarget;
    use desmos_bindings::reports::msg::ReportsMsg;
    use test_contract::msg::ExecuteMsg::DesmosMessages;

    #[test]
    fn test_create_report() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_report = ReportsMsg::CreateReport {
            subspace_id: TEST_SUBSPACE,
            reasons_ids: vec![TEST_REASON_ID],
            message: None,
            reporter: Addr::unchecked(&contract_address),
            target: ReportTarget::User {
                user: Addr::unchecked(USER1_ADDRESS),
            }
            .into(),
        };

        let msg = DesmosMessages {
            msgs: vec![create_report.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_delete_report() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let delete_report = ReportsMsg::DeleteReport {
            subspace_id: TEST_SUBSPACE,
            report_id: TEST_DELETABLE_REPORT_ID,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![delete_report.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_support_standard_reason() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let delete_report = ReportsMsg::SupportStandardReason {
            subspace_id: TEST_SUBSPACE,
            standard_reason_id: TEST_REASON_ID,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![delete_report.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_add_reason() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let add_reason = ReportsMsg::AddReason {
            subspace_id: TEST_SUBSPACE,
            title: "Test reason".to_string(),
            description: None,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![add_reason.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }

    #[test]
    fn test_remove_reason() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let remove_reason = ReportsMsg::RemoveReason {
            subspace_id: TEST_SUBSPACE,
            reason_id: TEST_DELETABLE_REASON_ID,
            signer: Addr::unchecked(&contract_address),
        };

        let msg = DesmosMessages {
            msgs: vec![remove_reason.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
