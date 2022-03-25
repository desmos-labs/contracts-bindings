use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesMsg {
    CreateSubspace {
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    },
    EditSubspace {
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    },
    DeleteSubspace {
        subspace_id: Uint64,
        signer: Addr,
    },
    CreateUserGroup {
        subspace_id: Uint64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    },
    EditUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    },
    SetUserGroupPermissions {
        subspace_id: Uint64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    },
    DeleteUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        signer: Addr,
    },
    AddUserToUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    },
    RemoveUserFromUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    },
    SetUserPermissions {
        subspace_id: Uint64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    },
}

impl SubspacesMsg {
    pub fn create_subspace(
        name: &str,
        description: &str,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateSubspace {
            name: name.to_owned(),
            description: description.to_owned(),
            treasury,
            owner,
            creator,
        }
    }

    pub fn edit_subspace(
        name: &str,
        description: &str,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditSubspace {
            name: name.to_owned(),
            description: description.to_owned(),
            treasury,
            owner,
            signer,
        }
    }

    pub fn delete_subspace(subspace_id: u64, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteSubspace {
            subspace_id: subspace_id.into(),
            signer,
        }
    }

    pub fn create_user_group(
        subspace_id: u64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateUserGroup {
            subspace_id: subspace_id.into(),
            name,
            description,
            default_permissions,
            creator,
        }
    }

    pub fn edit_user_group(
        subspace_id: u64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditUserGroup {
            subspace_id: subspace_id.into(),
            group_id,
            name,
            description,
            signer,
        }
    }

    pub fn set_user_group_permissions(
        subspace_id: u64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserGroupPermissions {
            subspace_id: subspace_id.into(),
            group_id,
            permissions,
            signer,
        }
    }

    pub fn delete_user_group(subspace_id: u64, group_id: u32, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteUserGroup {
            subspace_id: subspace_id.into(),
            group_id,
            signer,
        }
    }

    pub fn add_user_to_user_group(
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::AddUserToUserGroup {
            subspace_id: subspace_id.into(),
            group_id,
            user,
            signer,
        }
    }

    pub fn remove_user_from_user_group(
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::RemoveUserFromUserGroup {
            subspace_id: subspace_id.into(),
            group_id,
            user,
            signer,
        }
    }

    pub fn set_user_permissions(
        subspace_id: u64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserPermissions {
            subspace_id: subspace_id.into(),
            user,
            permissions,
            signer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Uint64;

    #[test]
    fn test_create_subspace() {
        let msg = SubspacesMsg::create_subspace(
            "test",
            "test",
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::CreateSubspace {
            name: "test".to_string(),
            description: "test".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_edit_subspace() {
        let msg = SubspacesMsg::edit_subspace(
            "test",
            "test",
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::EditSubspace {
            name: "test".to_string(),
            description: "test".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_delete_subspace() {
        let msg = SubspacesMsg::delete_subspace(
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::DeleteSubspace {
            subspace_id: Uint64::new(1),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_create_user_group() {
        let msg = SubspacesMsg::create_user_group(
            1,
            "test".to_string(),
            "test".to_string(),
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::CreateUserGroup {
            subspace_id: Uint64::new(1),
            name: "test".to_string(),
            description: "test".to_string(),
            default_permissions: 1,
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_edit_user_group() {
        let msg = SubspacesMsg::edit_user_group(
            1,
            1,
            "test".to_string(),
            "test".to_string(),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::EditUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            name: "test".to_string(),
            description: "test".to_string(),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_set_user_group_permissions() {
        let msg = SubspacesMsg::set_user_group_permissions(
            1,
            1,
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::SetUserGroupPermissions {
            subspace_id: Uint64::new(1),
            group_id: 1,
            permissions: 1,
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_delete_user_group() {
        let msg = SubspacesMsg::delete_user_group(
            1,
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::DeleteUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_add_user_to_user_group() {
        let msg = SubspacesMsg::add_user_to_user_group(
            1,
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = SubspacesMsg::AddUserToUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            user: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            signer: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_remove_user_to_user_group() {
        let msg = SubspacesMsg::remove_user_from_user_group(
            1,
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = SubspacesMsg::RemoveUserFromUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            user: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            signer: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_set_user_permissions() {
        let msg = SubspacesMsg::set_user_permissions(
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            1,
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = SubspacesMsg::SetUserPermissions {
            subspace_id: Uint64::new(1),
            user: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            permissions: 1,
            signer: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        assert_eq!(msg, expected)
    }
}
