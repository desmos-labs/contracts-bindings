//! Contains the query actions that can be sent to the chain in order to query data related
//! to the x/reports module.

use crate::legacy::reports::models::RawReportTarget;
use crate::legacy::reports::models_query::*;
use crate::legacy::types::PageRequest;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};

/// Query messages that can be sent to the x/reports module.
#[cw_serde]
#[derive(QueryResponses)]
pub enum ReportsQuery {
    /// Query the reports for a specific target.
    #[returns(QueryReportsResponse)]
    Reports {
        /// Id of the subspace to query the reports for.
        subspace_id: Uint64,
        /// Target to query the reports for.
        target: Option<RawReportTarget>,
        /// User that reported the target.  
        /// This is going to be used only if the `target` field is not `None`.
        reporter: Option<Addr>,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Query the report having the given id.
    #[returns(QueryReportResponse)]
    Report {
        /// Id of the subspace that holds the report to query for.
        subspace_id: Uint64,
        /// Id of the report to query for.
        report_id: Uint64,
    },
    /// Query the supported reporting reasons for a subspace.
    #[returns(QueryReasonsResponse)]
    Reasons {
        /// Id of the subspace to query the supported reporting reasons for.
        subspace_id: Uint64,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Query the reason having the given id.
    #[returns(QueryReasonResponse)]
    Reason {
        /// Id of the subspace that holds the reason to query for.
        subspace_id: Uint64,
        /// Id of the reason to query for.
        reason_id: u32,
    },
}
