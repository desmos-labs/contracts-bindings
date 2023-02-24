//! Contains the messages that can be sent to the chain to interact with the x/reports module.

use crate::reports::types::ReportTarget;
use crate::reports::types::*;
use cosmwasm_std::Addr;

/// ReportsMsg is the builder to generate Desmos x/reports messages.
pub enum ReportsMsg {}

impl ReportsMsg {
    /// Creates an instance of [`MsgCreateReport`].
    ///
    /// * `subspace_id` - Id of the subspace for which the report should be stored.
    /// * `reasons_ids` - Id of the reason this report has been created for.
    /// * `message` - Message attached to this report.
    /// * `reporter` - Address of the reporter.
    /// * `target` - Target of the report.
    pub fn create_report(
        subspace_id: u64,
        reasons_ids: Vec<u32>,
        message: &str,
        reporter: Addr,
        target: ReportTarget,
    ) -> MsgCreateReport {
        MsgCreateReport {
            subspace_id: subspace_id.into(),
            reasons_ids,
            message: message.into(),
            reporter: reporter.into(),
            target: Some(target.into()),
        }
    }

    /// Creates an instance of [`MsgDeleteReport`].
    ///
    /// * `subspace_id` - Id of the subspace that contains the report to be deleted.
    /// * `report_id` - Id of the report to be deleted.
    /// * `signer` - Address of the user deleting the report.
    pub fn delete_report(subspace_id: u64, report_id: u64, signer: Addr) -> MsgDeleteReport {
        MsgDeleteReport {
            subspace_id,
            report_id,
            signer: signer.into(),
        }
    }

    /// Creates an instance of [`MsgSupportStandardReason`].
    ///
    /// * `subspace_id` - Id of the subspace for which to support the reason
    /// * `standard_reason_id` - Id of the reason that should be supported
    /// * `signer` - Address of the user signing the message.
    pub fn support_standard_reason(
        subspace_id: u64,
        standard_reason_id: u32,
        signer: Addr,
    ) -> MsgSupportStandardReason {
        MsgSupportStandardReason {
            subspace_id,
            standard_reason_id,
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgAddReason`].
    ///
    /// * `subspace_id` - Id of the subspace for which to add the reason.
    /// * `title` - Title of the reason.
    /// * `description` - Extended description of the reason and the cases it applies to.
    /// * `signer` - Address of the user adding the supported reason.
    pub fn add_reason(
        subspace_id: u64,
        title: &str,
        description: &str,
        signer: Addr,
    ) -> MsgAddReason {
        MsgAddReason {
            subspace_id,
            title: title.into(),
            description: description.into(),
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgRemoveReason`].
    ///
    /// * `subspace_id` - Id of the subspace from which to remove the reason.
    /// * `reason_id` - Id of the reason to be deleted.
    /// * `signer` - Address of the user removing the supported reason.
    pub fn remove_reason(subspace_id: u64, reason_id: u32, signer: Addr) -> MsgRemoveReason {
        MsgRemoveReason {
            subspace_id,
            reason_id,
            signer: signer.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_report() {
        let msg = ReportsMsg::create_report(
            1,
            vec![0],
            "test",
            Addr::unchecked("reporter"),
            ReportTarget::Post(PostTarget { post_id: 1 }),
        );

        let expected = MsgCreateReport {
            subspace_id: 1,
            reasons_ids: vec![0],
            message: "test".into(),
            reporter: "reporter".into(),
            target: Some(ReportTarget::Post(PostTarget { post_id: 1 }).into()),
        };

        assert_eq!(expected, msg);
    }

    #[test]
    fn test_delete_report() {
        let msg = ReportsMsg::delete_report(1, 2, Addr::unchecked("reporter"));

        let expected = MsgDeleteReport {
            subspace_id: 1,
            report_id: 2,
            signer: "reporter".into(),
        };

        assert_eq!(expected, msg);
    }

    #[test]
    fn support_standard_reason() {
        let msg = ReportsMsg::support_standard_reason(1, 2, Addr::unchecked("reporter"));

        let expected = MsgSupportStandardReason {
            subspace_id: 1,
            standard_reason_id: 2,
            signer: "reporter".into(),
        };

        assert_eq!(expected, msg);
    }

    #[test]
    fn test_add_reason() {
        let msg = ReportsMsg::add_reason(
            1,
            "test reason",
            "Test description",
            Addr::unchecked("reporter"),
        );

        let expected = MsgAddReason {
            subspace_id: 1,
            title: "test reason".into(),
            description: "Test description".into(),
            signer: "reporter".into(),
        };

        assert_eq!(expected, msg);
    }

    #[test]
    fn test_remove_reason() {
        let msg = ReportsMsg::remove_reason(1, 2, Addr::unchecked("reporter"));

        let expected = MsgRemoveReason {
            subspace_id: 1,
            reason_id: 2,
            signer: "reporter".into(),
        };

        assert_eq!(expected, msg);
    }
}
