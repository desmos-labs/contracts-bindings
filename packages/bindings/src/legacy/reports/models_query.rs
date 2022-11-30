//! Contains structures returned from the [ReportsQuerier<'a>](crate::legacy::reports::querier::ReportsQuerier).

use crate::legacy::reports::models::{Reason, Report};
use crate::legacy::types::PageResponse;
use cosmwasm_schema::cw_serde;

/// Response to [`ReportsQuery::Reports`](crate::legacy::reports::query::ReportsQuery::Reports).
#[cw_serde]
pub struct QueryReportsResponse {
    /// Queried reports.
    pub reports: Vec<Report>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReportsQuery::Report`](crate::legacy::reports::query::ReportsQuery::Report).
#[cw_serde]
pub struct QueryReportResponse {
    /// Queried report.
    pub report: Report,
}

/// Response to [`ReportsQuery::Reasons`](crate::legacy::reports::query::ReportsQuery::Reasons).
#[cw_serde]
pub struct QueryReasonsResponse {
    /// Queried report reasons.
    pub reasons: Vec<Reason>,
    /// Details of the current fetched page.
    pub pagination: Option<PageResponse>,
}

/// Response to [`ReportsQuery::Reason`](crate::legacy::reports::query::ReportsQuery::Reason).
#[cw_serde]
pub struct QueryReasonResponse {
    /// Queried report reason.
    pub reason: Reason,
}
