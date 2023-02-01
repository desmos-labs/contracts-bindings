//! Contains the messages that can be sent to the chain to interact with the x/relationships module.

use crate::stargate::relationships::types::*;
use cosmwasm_std::Addr;

/// RelationshipsMsgBuilder is the builder to generate Desmos x/relationships messages.
pub struct RelationshipsMsgBuilder {}

impl RelationshipsMsgBuilder {
    /// Creates a new instance of [`MsgCreateRelationship`].
    ///
    /// * `signer` - Address of who wants to create the relationship.
    /// * `counterparty` - Address of the counterparty.
    /// * `subspace_id` - Subspace in which will be created the relationship.
    pub fn create_relationship(
        signer: Addr,
        counterparty: Addr,
        subspace_id: u64,
    ) -> MsgCreateRelationship {
        MsgCreateRelationship {
            signer: signer.into(),
            counterparty: counterparty.into(),
            subspace_id,
        }
    }

    /// Creates a new instance of [`MsgDeleteRelationship`].
    ///
    /// * `signer` - Address of who wants to delete the relationship.
    /// * `counterparty` - Address of the counterparty.
    /// * `subspace_id` - Subspace in which will be deleted the relationship.
    pub fn delete_relationship(
        signer: Addr,
        counterparty: Addr,
        subspace_id: u64,
    ) -> MsgDeleteRelationship {
        MsgDeleteRelationship {
            signer: signer.into(),
            counterparty: counterparty.into(),
            subspace_id,
        }
    }

    /// Creates a new instance of [`MsgBlockUser`].
    ///
    /// * `blocker` - Address of wants to create the block.
    /// * `blocked` - Address of the user that will be blocker from `blocker`.
    /// * `reason` - Reason of the block.
    /// * `subspace_id` - Subspace on which will be created the block.
    pub fn block_user(
        blocker: Addr,
        blocked: Addr,
        reason: &str,
        subspace_id: u64,
    ) -> MsgBlockUser {
        MsgBlockUser {
            blocker: blocker.into(),
            blocked: blocked.into(),
            reason: reason.into(),
            subspace_id,
        }
    }

    /// Creates a new instance of [`MsgUnblockUser`].
    ///
    /// * `blocker` - Address of who wants to delete the block.
    /// * `blocked` - Address of the user that will be unblocked from `blocker`.
    /// * `subspace_id` - Subspace in which will be deleted the block.
    pub fn unblock_user(blocker: Addr, blocked: Addr, subspace_id: u64) -> MsgUnblockUser {
        MsgUnblockUser {
            blocker: blocker.into(),
            blocked: blocked.into(),
            subspace_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_relationship() {
        let msg = RelationshipsMsgBuilder::create_relationship(
            Addr::unchecked("user"),
            Addr::unchecked("conterparty"),
            1,
        );
        let expected = MsgCreateRelationship {
            signer: "user".into(),
            counterparty: "conterparty".into(),
            subspace_id: 1,
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_relationship() {
        let msg = RelationshipsMsgBuilder::delete_relationship(
            Addr::unchecked("user"),
            Addr::unchecked("conterparty"),
            1,
        );
        let expected = MsgDeleteRelationship {
            signer: "user".into(),
            counterparty: "conterparty".into(),
            subspace_id: 1,
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_block_user() {
        let msg = RelationshipsMsgBuilder::block_user(
            Addr::unchecked("user"),
            Addr::unchecked("counterparty"),
            "test".into(),
            1,
        );
        let expected = MsgBlockUser {
            blocker: "user".into(),
            blocked: "counterparty".into(),
            reason: "test".into(),
            subspace_id: 1,
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_unblock_user() {
        let msg = RelationshipsMsgBuilder::unblock_user(
            Addr::unchecked("user"),
            Addr::unchecked("counterparty"),
            1,
        );
        let expected = MsgUnblockUser {
            blocker: "user".into(),
            blocked: "counterparty".into(),
            subspace_id: 1,
        };
        assert_eq!(expected, msg)
    }
}
