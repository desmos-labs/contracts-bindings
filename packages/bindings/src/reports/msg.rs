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

impl ReportsMsg {
    /// Creates an instance of [`ReportsMsg::CreateReport`].
    ///
    /// * `subspace_id` - Id of the subspace for which the report should be stored.
    /// * `reasons_id` - Id of the reason this report has been created for.
    /// * `message` - Message attached to this report.
    /// * `reporter` - Address of the reporter.
    /// * `target` - Target of the report.
    fn create_report(
        subspace_id: u64,
        reasons_id: Vec<u32>,
        message: Option<String>,
        reporter: Addr,
        target: ReportTarget,
    ) -> ReportsMsg {
        ReportsMsg::CreateReport {
            subspace_id: Uint64::new(subspace_id),
            reasons_id,
            message,
            reporter,
            target: target.into(),
        }
    }

    /// Creates an instance of [`ReportsMsg::DeleteReport`].
    ///
    /// * `subspace_id` - Id of the subspace that contains the report to be deleted.
    /// * `report_id` - Id of the report to be deleted.
    /// * `signer` - Address of the user deleting the report.
    fn delete_report(subspace_id: u64, report_id: u64, signer: Addr) -> ReportsMsg {
        ReportsMsg::DeleteReport {
            subspace_id: Uint64::new(subspace_id),
            report_id: Uint64::new(report_id),
            signer,
        }
    }

    /// Creates an instance of [`ReportsMsg::SupportStandardReason`].
    ///
    /// * `subspace_id` - Id of the subspace for which to support the reason
    /// * `standard_reason_id` - Id of the reason that should be supported
    /// * `signer` - Address of the user signing the message.
    fn support_standard_reason(
        subspace_id: u64,
        standard_reason_id: u32,
        signer: Addr,
    ) -> ReportsMsg {
        ReportsMsg::SupportStandardReason {
            subspace_id: Uint64::new(subspace_id),
            standard_reason_id,
            signer,
        }
    }

    /// Creates a new instance of [`ReportsMsg::AddReason`].
    ///
    /// * `subspace_id` - Id of the subspace for which to add the reason.
    /// * `title` - Title of the reason.
    /// * `description` - Extended description of the reason and the cases it applies to.
    /// * `signer` - Address of the user adding the supported reason.
    fn add_reason(
        subspace_id: u64,
        title: impl Into<String>,
        description: Option<String>,
        signer: Addr,
    ) -> ReportsMsg {
        ReportsMsg::AddReason {
            subspace_id: Uint64::new(subspace_id),
            title: title.into(),
            description,
            signer,
        }
    }

    /// Creates a new instance of [`ReportsMsg::RemoveReason`].
    ///
    /// * `subspace_id` - Id of the subspace from which to remove the reason.
    /// * `reason_id` - Id of the reason to be deleted.
    /// * `signer` - Address of the user removing the supported reason.
    fn remove_reason(subspace_id: u64, reason_id: u32, signer: Addr) -> ReportsMsg {
        ReportsMsg::RemoveReason {
            subspace_id: Uint64::new(subspace_id),
            reason_id,
            signer,
        }
    }
}
