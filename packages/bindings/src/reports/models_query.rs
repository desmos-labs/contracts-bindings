//! Contains structures returned from the [ReportsQuerier<'a>](crate::reports::querier::ReportsQuerier).

use crate::reports::models::{Reason, Report};
use crate::types::PageResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Response to [`ReportsQuery::Reports`](crate::reports::query::ReportsQuery::Reports).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReportsResponse {
    /// Queried reports.
    pub reports: Vec<Report>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReportsQuery::Report`](crate::reports::query::ReportsQuery::Report).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReportResponse {
    /// Queried report.
    pub report: Report,
}

/// Response to [`ReportsQuery::Reasons`](crate::reports::query::ReportsQuery::Reasons).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReasonsResponse {
    /// Queried report reasons.
    pub reasons: Vec<Reason>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReportsQuery::Reason`](crate::reports::query::ReportsQuery::Reason).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryReasonResponse {
    /// Queried report reason.
    pub reason: Reason,
}
