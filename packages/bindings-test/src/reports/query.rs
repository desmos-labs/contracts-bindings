#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_REASON_ID, TEST_REPORT_ID_WITH_USER_TARGET, TEST_SUBSPACE,
        TEST_SUBSPACE_EDITABLE_POST_ID, USER1_ADDRESS,
    };
    use cosmwasm_std::Addr;
    use desmos_bindings::reports::models::{RawReportTarget, ReportTarget};
    use desmos_bindings::reports::models_query::{
        QueryReasonResponse, QueryReasonsResponse, QueryReportResponse, QueryReportsResponse,
    };
    use desmos_bindings::reports::query::ReportsQuery;
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_reports() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Query all
        let query = DesmosChain {
            request: ReportsQuery::Reports {
                subspace_id: TEST_SUBSPACE,
                target: None,
                reporter: None,
                pagination: None,
            }
            .into(),
        };
        let response: QueryReportsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // There should be at least the 2 created test report
        assert!(response.reports.len() >= 2);

        // The first one should be the one reporting the user
        let report = response.reports.first().unwrap();
        assert_eq!(TEST_SUBSPACE, report.subspace_id);
        assert_eq!(Addr::unchecked(&contract_address), report.reporter);
        assert_eq!(
            RawReportTarget::from(ReportTarget::User {
                user: Addr::unchecked(USER1_ADDRESS)
            }),
            report.target
        );

        // The second one should be the one reporting a post
        let report = response.reports.get(1).unwrap();
        assert_eq!(TEST_SUBSPACE, report.subspace_id);
        assert_eq!(Addr::unchecked(&contract_address), report.reporter);
        assert_eq!(
            RawReportTarget::from(ReportTarget::Post {
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID
            }),
            report.target
        );

        // Query the post report
        let query = DesmosChain {
            request: ReportsQuery::Reports {
                subspace_id: TEST_SUBSPACE,
                target: Some(
                    ReportTarget::Post {
                        post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
                    }
                    .into(),
                ),
                reporter: None,
                pagination: None,
            }
            .into(),
        };
        let response: QueryReportsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // There should be only the post report in this response
        assert_eq!(1, response.reports.len());

        let report = response.reports.get(0).unwrap();
        assert_eq!(TEST_SUBSPACE, report.subspace_id);
        assert_eq!(Addr::unchecked(&contract_address), report.reporter);
        assert_eq!(
            RawReportTarget::from(ReportTarget::Post {
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID
            }),
            report.target
        );
    }

    #[test]
    fn test_query_report() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Query all
        let query = DesmosChain {
            request: ReportsQuery::Report {
                subspace_id: TEST_SUBSPACE,
                report_id: TEST_REPORT_ID_WITH_USER_TARGET,
            }
            .into(),
        };
        let response: QueryReportResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        assert_eq!(TEST_SUBSPACE, response.report.subspace_id);
        assert_eq!(
            RawReportTarget::from(ReportTarget::User {
                user: Addr::unchecked(USER1_ADDRESS)
            }),
            response.report.target
        );
    }

    #[test]
    fn test_query_reasons() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Query all
        let query = DesmosChain {
            request: ReportsQuery::Reasons {
                subspace_id: TEST_SUBSPACE,
                pagination: None,
            }
            .into(),
        };
        let response: QueryReasonsResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        // There should be at least the test reason
        assert!(response.reasons.len() >= 1);

        let reason = response.reasons.first().unwrap();

        assert_eq!("Test reason", reason.title.as_str());
        assert_eq!(TEST_REASON_ID, reason.id);
        assert_eq!(
            "Test reason description",
            reason.description.as_ref().unwrap().as_str()
        );
    }

    #[test]
    fn test_query_reason() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        // Query all
        let query = DesmosChain {
            request: ReportsQuery::Reason {
                subspace_id: TEST_SUBSPACE,
                reason_id: TEST_REASON_ID,
            }
            .into(),
        };
        let response: QueryReasonResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let reason = response.reason;

        assert_eq!("Test reason", reason.title.as_str());
        assert_eq!(TEST_REASON_ID, reason.id);
        assert_eq!(
            "Test reason description",
            reason.description.unwrap().as_str()
        );
    }
}
