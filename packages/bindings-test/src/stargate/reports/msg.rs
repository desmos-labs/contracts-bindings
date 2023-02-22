#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_DELETABLE_REASON_ID, TEST_DELETABLE_REPORT_ID, TEST_REASON_ID, TEST_SUBSPACE,
    };
    use cosmwasm_std::Addr;
    use desmos_bindings::stargate::reports::msg::ReportsMsgBuilder;
    use desmos_bindings::stargate::reports::types::{ReportTarget, UserTarget};
    use test_contract::msg::ExecuteMsg::DesmosMessages;

    #[test]
    fn test_create_report() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let create_report = ReportsMsgBuilder::create_report(
            TEST_SUBSPACE.into(),
            vec![TEST_REASON_ID],
            None,
            Addr::unchecked(&contract_address),
            ReportTarget::User(UserTarget {
                user: "desmos1ppvcentlcj2qzhzuu0zp2k492ef24asxmta5g5".into(),
            }),
        );

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

        let delete_report = ReportsMsgBuilder::delete_report(
            TEST_SUBSPACE.into(),
            TEST_DELETABLE_REPORT_ID.into(),
            Addr::unchecked(&contract_address),
        );

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

        let delete_report = ReportsMsgBuilder::support_standard_reason(
            TEST_SUBSPACE.into(),
            TEST_REASON_ID.into(),
            Addr::unchecked(&contract_address),
        );

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

        let add_reason = ReportsMsgBuilder::add_reason(
            TEST_SUBSPACE.into(),
            "Test reason".into(),
            None,
            Addr::unchecked(&contract_address),
        );

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

        let remove_reason = ReportsMsgBuilder::remove_reason(
            TEST_SUBSPACE.into(),
            TEST_DELETABLE_REASON_ID,
            Addr::unchecked(&contract_address),
        );

        let msg = DesmosMessages {
            msgs: vec![remove_reason.into()],
        };

        desmos_cli
            .wasm_execute(&contract_address, &msg)
            .assert_success();
    }
}
