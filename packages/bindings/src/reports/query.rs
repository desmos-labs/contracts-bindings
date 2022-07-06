//! Contains the query actions that can be sent to the chain in order to query data related
//! to the x/reports module.

use crate::reports::models::RawReportTarget;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Query messages that can be sent to the x/reports module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportsQuery {
    /// Query the reports for a specific target.
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
    Report {
        /// Id of the subspace that holds the report to query for.
        subspace_id: Uint64,
        /// Id of the report to query for.
        report_id: Uint64,
    },
    /// Query the supported reporting reasons for a subspace.
    Reasons {
        /// Id of the subspace to query the supported reporting reasons for.
        subspace_id: Uint64,
        /// Pagination configs.
        pagination: Option<PageRequest>,
    },
    /// Query the reason having the given id.
    Reason {
        /// Id of the subspace that holds the reason to query for.
        subspace_id: Uint64,
        /// Id of the reason to query for.
        reason_id: u32,
    },
}
