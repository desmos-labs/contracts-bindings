//! Contains useful mocks of the Desmos x/reports module's types made to be used in any test.

use crate::cosmos_types::Timestamp;
use crate::reports::types::{
    QueryReasonResponse, QueryReasonsResponse, QueryReportResponse, QueryReportsResponse, Reason,
    Report, UserTarget,
};

use chrono::DateTime;

/// Represents the mock reporter for unit test.
pub const MOCK_REPORTER: &str = "reporter";

/// Represents the mock user target of the report for unit test.
pub const MOCK_USER_TARGET: &str = "user_target";

/// Struct that contains some utility methods to mock data of the Desmos
/// x/reports module.
pub struct MockReportsQueries {}

impl MockReportsQueries {
    /// Gets a mocked instance of [`Report`].
    pub fn get_mocked_report(subspace_id: u64, id: u64) -> Report {
        Report {
            subspace_id,
            id,
            reasons_ids: vec![0, 2],
            message: "mock report".into(),
            reporter: MOCK_REPORTER.into(),
            target: Some(
                UserTarget {
                    user: MOCK_USER_TARGET.into(),
                }
                .into(),
            ),
            creation_date: Some(Timestamp::from(DateTime::from(
                DateTime::parse_from_rfc3339("2022-02-21T13:18:57.800827Z").unwrap(),
            ))),
        }
    }

    /// Gets a mocked instance of [`Reason`].
    pub fn get_mocked_reason(subspace_id: u64, id: u32) -> Reason {
        Reason {
            subspace_id,
            id,
            title: "Mock reason".into(),
            description: "mock reason".into(),
        }
    }

    /// Function that mocks a [`QueryReportsResponse`]
    pub fn get_mocked_reports_response() -> QueryReportsResponse {
        QueryReportsResponse {
            reports: vec![Self::get_mocked_report(1, 1)],
            pagination: None,
        }
    }

    /// Function that mocks a [`QueryReportsResponse`]
    pub fn get_mocked_report_response() -> QueryReportResponse {
        QueryReportResponse {
            report: Some(Self::get_mocked_report(1, 1)),
        }
    }

    /// Function that mocks a [`QueryReasonsResponse`]
    pub fn get_mocked_reasons_response() -> QueryReasonsResponse {
        QueryReasonsResponse {
            reasons: vec![Self::get_mocked_reason(1, 1)],
            pagination: None,
        }
    }

    /// Function that mocks a [`QueryReasonResponse`]
    pub fn get_mocked_reason_response() -> QueryReasonResponse {
        QueryReasonResponse {
            reason: Some(Self::get_mocked_reason(1, 1)),
        }
    }
}
