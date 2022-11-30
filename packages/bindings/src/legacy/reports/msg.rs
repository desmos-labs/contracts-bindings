//! Contains the messages that can be sent to the chain to interact with the x/reports module.

use crate::reports::models::{RawReportTarget, ReportTarget};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

/// Represents the messages to interact with the reports module.
#[cw_serde]
pub enum ReportsMsg {
    /// Represents the message to be used to create a report.
    CreateReport {
        /// Id of the subspace for which the report should be stored.
        subspace_id: Uint64,
        /// Id of the reason this report has been created for.
        reasons_ids: Vec<u32>,
        /// Message attached to this report.
        message: Option<String>,
        /// Address of the reporter.
        reporter: Addr,
        /// Target of the report.
        target: RawReportTarget,
    },
    /// Represents the message to be used when deleting a report.
    DeleteReport {
        /// Id of the subspace that contains the report to be deleted.
        subspace_id: Uint64,
        /// Id of the report to be deleted.
        report_id: Uint64,
        /// Address of the user deleting the report.
        signer: Addr,
    },
    /// Represents the message to be used when wanting to support one reason from the module params.
    SupportStandardReason {
        /// Id of the subspace for which to support the reason.
        subspace_id: Uint64,
        /// Id of the reason that should be supported.
        standard_reason_id: u32,
        /// Address of the user signing the message.
        signer: Addr,
    },
    /// Represents the message to be used when adding a new supported reason.
    AddReason {
        /// Id of the subspace for which to add the reason.
        subspace_id: Uint64,
        /// Title of the reason.
        title: String,
        /// Extended description of the reason and the cases it applies to.
        description: Option<String>,
        /// Address of the user adding the supported reason.
        signer: Addr,
    },
    /// Represents the message to be used when removing an exiting reporting reason.
    RemoveReason {
        /// Id of the subspace from which to remove the reason.
        subspace_id: Uint64,
        /// Id of the reason to be deleted.
        reason_id: u32,
        /// Address of the user removing the supported reason.
        signer: Addr,
    },
}

impl ReportsMsg {
    /// Creates an instance of [`ReportsMsg::CreateReport`].
    ///
    /// * `subspace_id` - Id of the subspace for which the report should be stored.
    /// * `reasons_ids` - Id of the reason this report has been created for.
    /// * `message` - Message attached to this report.
    /// * `reporter` - Address of the reporter.
    /// * `target` - Target of the report.
    pub fn create_report(
        subspace_id: u64,
        reasons_ids: Vec<u32>,
        message: Option<impl Into<String>>,
        reporter: Addr,
        target: ReportTarget,
    ) -> ReportsMsg {
        ReportsMsg::CreateReport {
            subspace_id: subspace_id.into(),
            reasons_ids,
            message: message.map(|message| message.into()),
            reporter,
            target: target.into(),
        }
    }

    /// Creates an instance of [`ReportsMsg::DeleteReport`].
    ///
    /// * `subspace_id` - Id of the subspace that contains the report to be deleted.
    /// * `report_id` - Id of the report to be deleted.
    /// * `signer` - Address of the user deleting the report.
    pub fn delete_report(subspace_id: u64, report_id: u64, signer: Addr) -> ReportsMsg {
        ReportsMsg::DeleteReport {
            subspace_id: subspace_id.into(),
            report_id: report_id.into(),
            signer,
        }
    }

    /// Creates an instance of [`ReportsMsg::SupportStandardReason`].
    ///
    /// * `subspace_id` - Id of the subspace for which to support the reason
    /// * `standard_reason_id` - Id of the reason that should be supported
    /// * `signer` - Address of the user signing the message.
    pub fn support_standard_reason(
        subspace_id: u64,
        standard_reason_id: u32,
        signer: Addr,
    ) -> ReportsMsg {
        ReportsMsg::SupportStandardReason {
            subspace_id: subspace_id.into(),
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
    pub fn add_reason(
        subspace_id: u64,
        title: impl Into<String>,
        description: Option<impl Into<String>>,
        signer: Addr,
    ) -> ReportsMsg {
        ReportsMsg::AddReason {
            subspace_id: subspace_id.into(),
            title: title.into(),
            description: description.map(|description| description.into()),
            signer,
        }
    }

    /// Creates a new instance of [`ReportsMsg::RemoveReason`].
    ///
    /// * `subspace_id` - Id of the subspace from which to remove the reason.
    /// * `reason_id` - Id of the reason to be deleted.
    /// * `signer` - Address of the user removing the supported reason.
    pub fn remove_reason(subspace_id: u64, reason_id: u32, signer: Addr) -> ReportsMsg {
        ReportsMsg::RemoveReason {
            subspace_id: subspace_id.into(),
            reason_id,
            signer,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::reports::models::ReportTarget;
    use crate::reports::msg::ReportsMsg;
    use cosmwasm_std::{Addr, Uint64};

    #[test]
    fn test_create_report() {
        let reports_msg = ReportsMsg::create_report(
            1,
            vec![0],
            Some("test".to_string()),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            ReportTarget::Post {
                post_id: Uint64::new(1),
            },
        );
        let expected = ReportsMsg::CreateReport {
            subspace_id: Uint64::new(1),
            reasons_ids: vec![0],
            message: Some("test".to_string()),
            reporter: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            target: ReportTarget::Post {
                post_id: Uint64::new(1),
            }
            .into(),
        };

        assert_eq!(expected, reports_msg);
    }

    #[test]
    fn test_delete_report() {
        let reports_msg = ReportsMsg::delete_report(
            1,
            2,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReportsMsg::DeleteReport {
            subspace_id: Uint64::new(1),
            report_id: Uint64::new(2),
            signer: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };

        assert_eq!(expected, reports_msg);
    }

    #[test]
    fn support_standard_reason() {
        let reports_msg = ReportsMsg::support_standard_reason(
            1,
            2,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReportsMsg::SupportStandardReason {
            subspace_id: Uint64::new(1),
            standard_reason_id: 2,
            signer: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };

        assert_eq!(expected, reports_msg);
    }

    #[test]
    fn test_add_reason() {
        let reports_msg = ReportsMsg::add_reason(
            1,
            "test reason",
            Some("Test description".to_string()),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReportsMsg::AddReason {
            subspace_id: Uint64::new(1),
            title: "test reason".to_string(),
            description: Some("Test description".to_string()),
            signer: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };

        assert_eq!(expected, reports_msg);
    }

    #[test]
    fn test_remove_reason() {
        let reports_msg = ReportsMsg::remove_reason(
            1,
            2,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReportsMsg::RemoveReason {
            subspace_id: Uint64::new(1),
            reason_id: 2,
            signer: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };

        assert_eq!(expected, reports_msg);
    }
}
