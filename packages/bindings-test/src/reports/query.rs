#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_REASON_ID, TEST_REPORT_ID_WITH_USER_TARGET, TEST_SUBSPACE,
        TEST_SUBSPACE_EDITABLE_POST_ID, USER1_ADDRESS,
    };
    use cosmwasm_std::Addr;
    use desmos_bindings::reports::types::{PostTarget, UserTarget};
    use desmos_bindings::reports::types::{
        QueryReasonRequest, QueryReasonResponse, QueryReasonsRequest, QueryReasonsResponse,
        QueryReportRequest, QueryReportResponse, QueryReportsRequest, QueryReportsResponse,
    };
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_query_reports() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        // Query all
        let query = DesmosChain {
            request: QueryReportsRequest {
                subspace_id: TEST_SUBSPACE,
                target: None,
                reporter: "".into(),
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
        assert_eq!(&contract_address, report.reporter.as_str());
        assert_eq!(
            UserTarget {
                user: USER1_ADDRESS.into()
            },
            UserTarget::try_from(report.target.clone().unwrap()).unwrap(),
        );

        // The second one should be the one reporting a post
        let report = response.reports.get(1).unwrap();
        assert_eq!(TEST_SUBSPACE, report.subspace_id);
        assert_eq!(Addr::unchecked(&contract_address), report.reporter);
        assert_eq!(
            PostTarget {
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID
            },
            PostTarget::try_from(report.target.clone().unwrap()).unwrap(),
        );

        // Query the post report
        let query = DesmosChain {
            request: QueryReportsRequest {
                subspace_id: TEST_SUBSPACE,
                target: Some(
                    PostTarget {
                        post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
                    }
                    .into(),
                ),
                reporter: "".into(),
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
            PostTarget {
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID
            },
            PostTarget::try_from(report.target.clone().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_query_report() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        // Query all
        let query = DesmosChain {
            request: QueryReportRequest {
                subspace_id: TEST_SUBSPACE,
                report_id: TEST_REPORT_ID_WITH_USER_TARGET.into(),
            }
            .into(),
        };
        let response: QueryReportResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let report = response.report.unwrap();
        assert_eq!(TEST_SUBSPACE, report.subspace_id);
        assert_eq!(
            UserTarget {
                user: USER1_ADDRESS.into()
            },
            UserTarget::try_from(report.target.unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_query_reasons() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        // Query all
        let query = DesmosChain {
            request: QueryReasonsRequest {
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
        assert_eq!("Test reason description", reason.description.as_str());
    }

    #[test]
    fn test_query_reason() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1, 0);

        // Query all
        let query = DesmosChain {
            request: QueryReasonRequest {
                subspace_id: TEST_SUBSPACE,
                reason_id: TEST_REASON_ID,
            }
            .into(),
        };
        let response: QueryReasonResponse =
            desmos_cli.wasm_query(&contract_address, &query).to_object();

        let reason = response.reason.unwrap();

        assert_eq!("Test reason", reason.title.as_str());
        assert_eq!(TEST_REASON_ID, reason.id);
        assert_eq!("Test reason description", reason.description.as_str());
    }
}
