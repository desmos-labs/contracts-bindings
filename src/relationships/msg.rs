//! Contains the messages that can be sent to the chain to interact with the x/relationships module.

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the messages to interact with x/relatioships module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipsMsg {
    /// Message to create a new relationship.
    CreateRelationship {
        /// Address of who wants to create the relationship.
        signer: Addr,
        /// Address of the counterparty.
        counterparty: Addr,
        /// Subspace in which will be created the relationship.
        subspace_id: Uint64,
    },
    /// Message to delete a relationship.
    DeleteRelationship {
        /// Address of who wants to delete the relationship.
        signer: Addr,
        /// Address of the counterparty.
        counterparty: Addr,
        /// Subspace in which will be deleted the relationship.
        subspace_id: Uint64,
    },
    /// Message to block another user.
    BlockUser {
        /// Address of who is creating the block.
        blocker: Addr,
        /// Address of the user that will be blocked from `blocker`.
        blocked: Addr,
        /// Block reason.
        reason: String,
        /// Subspace in which will be created the block.
        subspace_id: Uint64,
    },
    /// Message to delete a previously created block.
    UnblockUser {
        /// Address of who wants to remove the block.
        blocker: Addr,
        /// Address of the user that will be unblocked from `blocker`.
        blocked: Addr,
        /// Subspace in which will be deleted the block.
        subspace_id: Uint64,
    },
}

impl RelationshipsMsg {
    /// Creates a new instance of [`RelationshipsMsg::CreateRelationship`].
    ///
    /// * `signer` - Address of who wants to create the relationship.
    /// * `counterparty` - Address of the counterparty.
    /// * `subspace_id` - Subspace in which will be created the relationship.
    pub fn create_relationship(
        signer: Addr,
        counterparty: Addr,
        subspace_id: u64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::CreateRelationship {
            signer,
            counterparty,
            subspace_id: subspace_id.into(),
        }
    }

    /// Creates a new instance of [`RelationshipsMsg::DeleteRelationship`].
    ///
    /// * `user` - Address of who wants to delete the relationship.
    /// * `counterparty` - Address of the counterparty.
    /// * `subspace_id` - Subspace in which will be deleted the relationship.
    pub fn delete_relationship(
        user: Addr,
        counterparty: Addr,
        subspace_id: u64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::DeleteRelationship {
            signer: user,
            counterparty,
            subspace_id: subspace_id.into(),
        }
    }

    /// Creates a new instance of [`RelationshipsMsg::BlockUser`].
    ///
    /// * `blocker` - Address of wants to create the block.
    /// * `blocked` - Address of the user that will be blocker from `blocker`.
    /// * `reason` - Reason of the block.
    /// * `subspace_id` - Subspace on which will be created the block.
    pub fn block_user(
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace_id: u64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::BlockUser {
            blocker,
            blocked,
            reason,
            subspace_id: subspace_id.into(),
        }
    }

    /// Creates a new instance of [`RelationshipsMsg::UnblockUser`].
    ///
    /// * `blocker` - Address of who wants to delete the block.
    /// * `blocked` - Address of the user that will be unblocked from `blocker`.
    /// * `subspace_id` - Subspace in which will be deleted the block.
    pub fn unblock_user(blocker: Addr, blocked: Addr, subspace_id: u64) -> RelationshipsMsg {
        RelationshipsMsg::UnblockUser {
            blocker,
            blocked,
            subspace_id: subspace_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::relationships::msg::RelationshipsMsg;
    use cosmwasm_std::{Addr, Uint64};

    #[test]
    fn test_create_relationship() {
        let msg = RelationshipsMsg::create_relationship(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            1,
        );
        let expected = RelationshipsMsg::CreateRelationship {
            signer: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            counterparty: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace_id: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_relationship() {
        let msg = RelationshipsMsg::delete_relationship(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            1,
        );
        let expected = RelationshipsMsg::DeleteRelationship {
            signer: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            counterparty: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace_id: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_block_user() {
        let msg = RelationshipsMsg::block_user(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            "test".to_string(),
            1,
        );
        let expected = RelationshipsMsg::BlockUser {
            blocker: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            blocked: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            reason: "test".to_string(),
            subspace_id: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_unblock_user() {
        let msg = RelationshipsMsg::unblock_user(
            Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            1,
        );
        let expected = RelationshipsMsg::UnblockUser {
            blocker: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
            blocked: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            subspace_id: Uint64::new(1),
        };
        assert_eq!(expected, msg)
    }
}
