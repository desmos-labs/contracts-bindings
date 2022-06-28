use crate::reports::models::RawReportTarget;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportsMsg {
    CreateReport {
        subspace_id: Uint64,
        reasons_id: Vec<u32>,
        message: Option<String>,
        reporter: Addr,
        target: RawReportTarget,
    },
    DeleteReport {
        subspace_id: Uint64,
        report_id: Uint64,
        signer: Addr,
    },
    SupportStandardReason {
        subspace_id: Uint64,
        standard_reason_id: u32,
        signer: Addr,
    },
    AddReason {
        subspace_id: Uint64,
        title: String,
        description: Option<String>,
        signer: Addr,
    },
    RemoveReason {
        subspace_id: Uint64,
        reason_id: u32,
        signer: Addr,
    },
}
