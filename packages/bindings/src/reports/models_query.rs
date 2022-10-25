//! Contains structures returned from the [ReportsQuerier<'a>](crate::reports::querier::ReportsQuerier).

use crate::reports::models::{Reason, Report};
use crate::types::PageResponse;
use cosmwasm_schema::cw_serde;

/// Response to [`ReportsQuery::Reports`](crate::reports::query::ReportsQuery::Reports).
#[cw_serde]
pub struct QueryReportsResponse {
    /// Queried reports.
    pub reports: Vec<Report>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReportsQuery::Report`](crate::reports::query::ReportsQuery::Report).
#[cw_serde]
pub struct QueryReportResponse {
    /// Queried report.
    pub report: Report,
}

/// Response to [`ReportsQuery::Reasons`](crate::reports::query::ReportsQuery::Reasons).
#[cw_serde]
pub struct QueryReasonsResponse {
    /// Queried report reasons.
    pub reasons: Vec<Reason>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReportsQuery::Reason`](crate::reports::query::ReportsQuery::Reason).
#[cw_serde]
pub struct QueryReasonResponse {
    /// Queried report reason.
    pub reason: Reason,
}
