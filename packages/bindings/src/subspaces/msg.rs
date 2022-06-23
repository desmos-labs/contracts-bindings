//! Contains the messages that can be sent to the chain to interact with the x/subspaces module.

use crate::subspaces::models::Permission;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the messages to interact with the x/subspaces module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesMsg {
    /// Message to create a new subspace.
    CreateSubspace {
        /// Subspace name.
        name: String,
        /// Subspace description.
        description: String,
        /// Subspace treasury address.
        /// Represents the address that will pay for the fees
        /// needed to performs application links.
        treasury: Addr,
        /// Subspace owner.
        owner: Addr,
        /// Subspace creator.
        creator: Addr,
    },
    /// Message to modify a subspace.
    EditSubspace {
        /// Id of the subspace to edit.
        subspace_id: Uint64,
        /// New subspace name.
        name: String,
        /// New subspace description.
        description: String,
        /// New subspace treasury address.
        /// Represents the address that will pay for the fees
        /// needed to performs application links.
        treasury: Addr,
        /// New subspace owner.
        owner: Addr,
        /// Address of who wants to edit the subspace.
        signer: Addr,
    },
    /// Message to delete a subspace.
    DeleteSubspace {
        /// Id of the subspace to delete.
        subspace_id: Uint64,
        /// Address of who wants delete the subspace.
        signer: Addr,
    },
    /// Message to create a subspace section.
    CreateSection {
        /// Id of the subspace inside which the section will be placed.
        subspace_id: Uint64,
        /// Name of the section to be created.
        name: String,
        /// Description of the section.
        description: Option<String>,
        /// Id of the parent section.
        parent_id: Option<u32>,
        /// User creating the section.
        creator: Addr,
    },
    /// Message to edit a subspace section.
    EditSection {
        /// Id of the subspace inside which the section to be edited is.
        subspace_id: Uint64,
        /// Id of the section to be edited.
        section_id: u32,
        /// New name of the section.
        name: Option<String>,
        /// New description of the section.
        description: Option<String>,
        /// User editing the section.
        editor: Addr,
    },
    /// Message to move a subspace section.
    MoveSection {
        /// Id of the subspace inside which the section lies.
        subspace_id: Uint64,
        /// Id of the section to be moved.
        section_id: u32,
        /// Id of the new parent.
        new_parent_id: u32,
        /// User moving the section.
        signer: Addr,
    },
    /// Message to delete a subspace section.
    DeleteSection {
        /// Id of the subspace inside which the section to be deleted is.
        subspace_id: Uint64,
        /// Id of the section to delete.
        section_id: u32,
        /// User deleting the section.
        signer: Addr,
    },
    /// Message to create a new user group.
    CreateUserGroup {
        /// Subspace id to which the group will belong.
        subspace_id: Uint64,
        /// Id of the section inside which the group will be created.
        section_id: Option<u32>,
        /// Name of the user group.
        name: String,
        /// Description of the user group.
        description: Option<String>,
        /// Permissions that all the members will inherit.
        default_permissions: Vec<Permission>,
        /// Address of who wants create the user group.
        creator: Addr,
    },
    /// Message to modify a user group.
    EditUserGroup {
        /// Subspace id to which the user group belongs.
        subspace_id: Uint64,
        /// Id of the user group to edit.
        group_id: u32,
        /// New user group name.
        name: Option<String>,
        /// New user group description.
        description: Option<String>,
        /// Address of who wants edit the user group.
        signer: Addr,
    },
    /// Message to move a user group to from a section to another.
    MoveUserGroup {
        /// Id of the subspace inside which the group to move is.
        subspace_id: Uint64,
        /// Id of the group to be moved.
        group_id: u32,
        /// Id of the new section where to move the group.
        new_section_id: u32,
        /// Address of who wants move the user group.
        signer: Addr,
    },
    /// Message to update the permissions that the group members will inherit.
    SetUserGroupPermissions {
        /// Subspace id to which the user group belongs.
        subspace_id: Uint64,
        /// Id of the group of interest.
        group_id: u32,
        /// The permissions that will be set for the user group.
        permissions: Vec<Permission>,
        /// Address of who wants update the user group permissions.
        signer: Addr,
    },
    /// Message to delete a user group.
    DeleteUserGroup {
        /// Subspace id to which the group belongs.
        subspace_id: Uint64,
        /// Id of the group to delete.
        group_id: u32,
        /// Address of who wants delete the user group.
        signer: Addr,
    },
    /// Message to add a new user to a group.
    AddUserToUserGroup {
        /// Subspace id to which belong the group.
        subspace_id: Uint64,
        /// Id of the group to which will be added the user.
        group_id: u32,
        /// Address of the user to add to the group.
        user: Addr,
        /// Address of who wants add a new user to the group.
        signer: Addr,
    },
    /// Message to remove a user from a group.
    RemoveUserFromUserGroup {
        /// Subspace id to which the group belongs.
        subspace_id: Uint64,
        /// Id of the group from where will be removed the user.
        group_id: u32,
        /// Address of the user that will be removed.
        user: Addr,
        /// Address of who wants remove the user from the group.
        signer: Addr,
    },
    /// Sets the permissions that an user have inside a subspace.
    SetUserPermissions {
        /// Subspace id to which the user belongs.
        subspace_id: Uint64,
        /// Id of the section for which to set the permissions.
        section_id: u32,
        /// Address of the user.
        user: Addr,
        /// The new user's permissions.
        permissions: Vec<Permission>,
        /// Address of who wants update the user's permissions.
        signer: Addr,
    },
}

impl SubspacesMsg {
    /// Creates a new instance of [`SubspacesMsg::CreateSubspace`].
    ///
    /// * `name` - Subspace name.
    /// * `description` - Subspace description.
    /// * `treasury` - Treasury address.
    /// Represents the address that will pay for the fees
    /// needed to performs application links.
    /// * `owner` - Address of who will be the subspace owner.
    /// * `creator` - Address of who wants to create the subspace.
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

    /// Creates a new instance of [`SubspacesMsg::EditSubspace`].
    ///
    /// * `subspace_id` - Id of the subspace to edit.
    /// * `name` - New subspace name.
    /// * `description` - New subspace description.
    /// * `treasury` - New subspace treasury.
    /// Represents the address that will pay for the fees
    /// needed to performs application links.
    /// * `signer` - Address of who wants edit the subspace.
    pub fn edit_subspace(
        subspace_id: u64,
        name: &str,
        description: &str,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditSubspace {
            subspace_id: subspace_id.into(),
            name: name.to_owned(),
            description: description.to_owned(),
            treasury,
            owner,
            signer,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::DeleteSubspace`].
    ///
    /// * `subspace_id` - id of the subspace to delete.
    /// * `signer` - Address of who wants delete the subsapce.
    pub fn delete_subspace(subspace_id: u64, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteSubspace {
            subspace_id: subspace_id.into(),
            signer,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::CreateSection`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the section will be placed.
    /// * `name` - Name of the section to be created.
    /// * `description` - Description of the section.
    /// * `parent_id` - Id of the parent section.
    /// * `creator` - User creating the section.
    pub fn create_section(
        subspace_id: u64,
        name: impl Into<String>,
        description: Option<String>,
        parent_id: Option<u32>,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateSection {
            subspace_id: Uint64::new(subspace_id),
            name: name.into(),
            description,
            parent_id,
            creator,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::EditSection`].
    ///
    /// `subspace_id` - Id of the subspace inside which the section to be edited is.
    /// `section_id` - Id of the section to be edited.
    /// `name` - New name of the section.
    /// `description` - New description of the section.
    /// `editor` - User editing the section.
    pub fn edit_section(
        subspace_id: u64,
        section_id: u32,
        name: Option<String>,
        description: Option<String>,
        editor: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditSection {
            subspace_id: Uint64::new(subspace_id),
            section_id,
            name,
            description,
            editor,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::MoveSection`].
    ///
    /// `subspace_id` - Id of the subspace inside which the section lies.
    /// `section_id` - Id of the section to be moved.
    /// `new_parent_id` - Id of the new parent.
    /// `signer` - User moving the section.
    pub fn move_section(
        subspace_id: u64,
        section_id: u32,
        new_parent_id: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::MoveSection {
            subspace_id: Uint64::new(subspace_id),
            section_id,
            new_parent_id,
            signer,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::DeleteSection`].
    ///
    /// `subspace_id` - Id of the subspace inside which the section to be deleted is.
    /// `section_id` - Id of the section to delete.
    /// `signer` - User deleting the section.
    pub fn delete_section(subspace_id: u64, section_id: u32, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteSection {
            subspace_id: Uint64::new(subspace_id),
            section_id,
            signer,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::CreateUserGroup`].
    ///
    /// * `subspace_id` - Subspace id to which the group will belong.
    /// * `section_id` - Id of the section inside which the group will be created.
    /// * `name` - Group name.
    /// * `description` - Group description.
    /// * `default_permission` - Permissions that the members will inherit.
    /// * `creator` - Address of who wants to create the group.
    pub fn create_user_group(
        subspace_id: u64,
        section_id: Option<u32>,
        name: String,
        description: Option<String>,
        default_permissions: Vec<Permission>,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateUserGroup {
            subspace_id: subspace_id.into(),
            section_id,
            name,
            description,
            default_permissions,
            creator,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::EditUserGroup`].
    ///
    /// * `subspace_id` - Subspace id to which the group belongs.
    /// * `group_id` - Id of the group to edit.
    /// * `name` - New group name.
    /// * `description` - New group description.
    /// * `signer` - Address of who wants edit the user group.
    pub fn edit_user_group(
        subspace_id: u64,
        group_id: u32,
        name: Option<String>,
        description: Option<String>,
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

    /// Creates a new instance of [`SubspacesMsg::MoveUserGroup`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the group to move is.
    /// * `group_id` - Id of the group to be moved.
    /// * `new_section_id` - d of the new section where to move the group.
    /// * `signer` - Address of who wants move the user group.
    pub fn move_user_group(
        subspace_id: u64,
        group_id: u32,
        new_section_id: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::MoveUserGroup {
            subspace_id: Uint64::new(subspace_id),
            group_id,
            new_section_id,
            signer,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::SetUserGroupPermissions`].
    ///
    /// * `subspace_id` - Subspace to which the user group belongs.
    /// * `group_id` - Id of user group of interest.
    /// * `permissions` - The new permissions that will be set to the group.
    /// * `signer` - Address of who wants set the group permissions.
    pub fn set_user_group_permissions(
        subspace_id: u64,
        group_id: u32,
        permissions: Vec<Permission>,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserGroupPermissions {
            subspace_id: subspace_id.into(),
            group_id,
            permissions,
            signer,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::DeleteUserGroup`].
    ///
    /// * `subspace_id` - Id of the subspace to which the group belongs.
    /// * `group_id` - Id of the group to delete.
    /// * `signer` - Address of who wants to delete the group.
    pub fn delete_user_group(subspace_id: u64, group_id: u32, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteUserGroup {
            subspace_id: subspace_id.into(),
            group_id,
            signer,
        }
    }

    /// Creates a new instance of [`SubspacesMsg::AddUserToUserGroup`].
    ///
    /// * `subspace_id` - Subspace id to which the group belongs.
    /// * `group_id` - Id of the group to which will be added the user.
    /// * `user` - Address of the user to add to the group.
    /// * `signer` - Address of who wants to add the user to the group.
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

    /// Creates a new instance of [`SubspacesMsg::RemoveUserFromUserGroup`].
    ///
    /// * `subspace_id` - Subspace id to which the group belongs.
    /// * `group_id` - Id of the group from which will be removed the user.
    /// * `user` - Address of the user to remove from the group.
    /// * `signer` - Address of who wants to remove the user from the group.
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

    /// Creates a new instance of [`SubspacesMsg::SetUserPermissions`].
    ///
    /// * `subspace_id` - Subspace id to which the user belongs.
    /// * `section_id` - Id of the section for which to set the permissions.
    /// * `user` - User address.
    /// * `permissions` - New user's permissions.
    /// * `signer` - Address of who wants to update the user's permissions.
    pub fn set_user_permissions(
        subspace_id: u64,
        section_id: u32,
        user: Addr,
        permissions: Vec<Permission>,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserPermissions {
            subspace_id: subspace_id.into(),
            section_id,
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
            42,
            "test",
            "test",
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::EditSubspace {
            subspace_id: Uint64::new(42),
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
    fn test_create_section() {
        let msg = SubspacesMsg::create_section(
            1,
            "test_section",
            Some("test description".to_string()),
            Some(1),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::CreateSection {
            subspace_id: Uint64::new(1),
            name: "test_section".to_string(),
            description: Some("test description".to_string()),
            parent_id: Some(1),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_edit_section() {
        let msg = SubspacesMsg::edit_section(
            1,
            1,
            Some("test_section".to_string()),
            Some("test description".to_string()),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::EditSection {
            subspace_id: Uint64::new(1),
            section_id: 1,
            name: Some("test_section".to_string()),
            description: Some("test description".to_string()),
            editor: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_move_section() {
        let msg = SubspacesMsg::move_section(
            1,
            1,
            2,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::MoveSection {
            subspace_id: Uint64::new(1),
            section_id: 1,
            new_parent_id: 2,
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_delete_section() {
        let msg = SubspacesMsg::delete_section(
            1,
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::DeleteSection {
            subspace_id: Uint64::new(1),
            section_id: 1,
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_create_user_group() {
        let msg = SubspacesMsg::create_user_group(
            1,
            Some(1),
            "test".to_string(),
            Some("test".to_string()),
            vec![],
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::CreateUserGroup {
            subspace_id: Uint64::new(1),
            section_id: Some(1),
            name: "test".to_string(),
            description: Some("test".to_string()),
            default_permissions: vec![],
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_edit_user_group() {
        let msg = SubspacesMsg::edit_user_group(
            1,
            1,
            Some("test".to_string()),
            Some("test".to_string()),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::EditUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            name: Some("test".to_string()),
            description: Some("test".to_string()),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_move_user_group() {
        let msg = SubspacesMsg::move_user_group(
            1,
            1,
            2,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::MoveUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            new_section_id: 2,
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_set_user_group_permissions() {
        let msg = SubspacesMsg::set_user_group_permissions(
            1,
            1,
            vec![],
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::SetUserGroupPermissions {
            subspace_id: Uint64::new(1),
            group_id: 1,
            permissions: vec![],
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
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            vec![],
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = SubspacesMsg::SetUserPermissions {
            subspace_id: Uint64::new(1),
            section_id: 1,
            user: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            permissions: vec![],
            signer: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        assert_eq!(msg, expected)
    }
}
