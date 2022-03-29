use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipsMsg {
    CreateRelationship {
        signer: Addr,
        counterparty: Addr,
        subspace_id: Uint64,
    },
    DeleteRelationship {
        signer: Addr,
        counterparty: Addr,
        subspace_id: Uint64,
    },
    BlockUser {
        blocker: Addr,
        blocked: Addr,
        reason: String,
        subspace_id: Uint64,
    },
    UnblockUser {
        blocker: Addr,
        blocked: Addr,
        subspace_id: Uint64,
    },
}

impl RelationshipsMsg {
    pub fn create_relationship(
        sender: Addr,
        counterparty: Addr,
        subspace_id: u64,
    ) -> RelationshipsMsg {
        RelationshipsMsg::CreateRelationship {
            signer: sender,
            counterparty,
            subspace_id: subspace_id.into(),
        }
    }

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
