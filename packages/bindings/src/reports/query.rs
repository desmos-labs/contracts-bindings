use crate::reports::models::RawReportTarget;
use crate::types::PageRequest;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportsQuery {
    Reports {
        subspace_id: Uint64,
        target: Option<RawReportTarget>,
        reporter: Option<Addr>,
        pagination: Option<PageRequest>,
    },
    Report {
        subspace_id: Uint64,
        report_id: Uint64,
    },
    Reasons {
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
    Reason {
        subspace_id: Uint64,
        reason_id: u32,
    },
}
