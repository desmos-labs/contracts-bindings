//! Contains the messages that can be sent to the chain to interact with the x/subspaces module.

use crate::subspaces::types::Permission;
use crate::subspaces::types::*;
use cosmwasm_std::Addr;

/// SubspacesMsg is the builder to generate Desmos x/subspaces messages.
pub struct SubspacesMsg {}

impl SubspacesMsg {
    /// Creates a new instance of [`MsgCreateSubspace`].
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
        owner: Addr,
        creator: Addr,
    ) -> MsgCreateSubspace {
        MsgCreateSubspace {
            name: name.into(),
            description: description.into(),
            owner: owner.into(),
            creator: creator.into(),
        }
    }

    /// Creates a new instance of [`MsgEditSubspace`].
    ///
    /// * `subspace_id` - Id of the subspace to edit.
    /// * `name` - New subspace name.
    /// * `description` - New subspace description.
    /// * `treasury` - New subspace treasury.
    /// * `owner` - Address of who will be the subspace owner.
    /// * `signer` - Address of who wants edit the subspace.
    pub fn edit_subspace(
        subspace_id: u64,
        name: &str,
        description: &str,
        owner: Addr,
        signer: Addr,
    ) -> MsgEditSubspace {
        MsgEditSubspace {
            subspace_id,
            name: name.into(),
            description: description.into(),
            owner: owner.into(),
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgDeleteSubspace`].
    ///
    /// * `subspace_id` - id of the subspace to delete.
    /// * `signer` - Address of who wants delete the subsapce.
    pub fn delete_subspace(subspace_id: u64, signer: Addr) -> MsgDeleteSubspace {
        MsgDeleteSubspace {
            subspace_id,
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgCreateSection`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the section will be placed.
    /// * `name` - Name of the section to be created.
    /// * `description` - Description of the section.
    /// * `parent_id` - Id of the parent section.
    /// * `creator` - User creating the section.
    pub fn create_section(
        subspace_id: u64,
        name: &str,
        description: Option<&str>,
        parent_id: Option<u32>,
        creator: Addr,
    ) -> MsgCreateSection {
        MsgCreateSection {
            subspace_id,
            name: name.into(),
            description: description.unwrap_or_default().into(),
            parent_id: parent_id.unwrap_or_default(),
            creator: creator.into(),
        }
    }

    /// Creates a new instance of [`MsgEditSection`].
    ///
    /// `subspace_id` - Id of the subspace inside which the section to be edited is.
    /// `section_id` - Id of the section to be edited.
    /// `name` - New name of the section.
    /// `description` - New description of the section.
    /// `editor` - User editing the section.
    pub fn edit_section(
        subspace_id: u64,
        section_id: u32,
        name: Option<&str>,
        description: Option<&str>,
        editor: Addr,
    ) -> MsgEditSection {
        MsgEditSection {
            subspace_id,
            section_id,
            name: name.unwrap_or_default().into(),
            description: description.unwrap_or_default().into(),
            editor: editor.into(),
        }
    }

    /// Creates a new instance of [`MsgMoveSection`].
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
    ) -> MsgMoveSection {
        MsgMoveSection {
            subspace_id,
            section_id,
            new_parent_id,
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgDeleteSection`].
    ///
    /// `subspace_id` - Id of the subspace inside which the section to be deleted is.
    /// `section_id` - Id of the section to delete.
    /// `signer` - User deleting the section.
    pub fn delete_section(subspace_id: u64, section_id: u32, signer: Addr) -> MsgDeleteSection {
        MsgDeleteSection {
            subspace_id,
            section_id,
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgCreateUserGroup`].
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
        name: &str,
        description: Option<&str>,
        default_permissions: Vec<Permission>,
        initial_members: Vec<Addr>,
        creator: Addr,
    ) -> MsgCreateUserGroup {
        MsgCreateUserGroup {
            subspace_id,
            section_id: section_id.unwrap_or_default(),
            name: name.into(),
            description: description.unwrap_or_default().into(),
            default_permissions: default_permissions.into_iter().map(Into::into).collect(),
            initial_members: initial_members.into_iter().map(Into::into).collect(),
            creator: creator.into(),
        }
    }

    /// Creates a new instance of [`MsgEditUserGroup`].
    ///
    /// * `subspace_id` - Subspace id to which the group belongs.
    /// * `group_id` - Id of the group to edit.
    /// * `name` - New group name.
    /// * `description` - New group description.
    /// * `signer` - Address of who wants edit the user group.
    pub fn edit_user_group(
        subspace_id: u64,
        group_id: u32,
        name: Option<&str>,
        description: Option<&str>,
        signer: Addr,
    ) -> MsgEditUserGroup {
        MsgEditUserGroup {
            subspace_id,
            group_id,
            name: name.unwrap_or_default().into(),
            description: description.unwrap_or_default().into(),
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgMoveUserGroup`].
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
    ) -> MsgMoveUserGroup {
        MsgMoveUserGroup {
            subspace_id,
            group_id,
            new_section_id,
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgSetUserGroupPermissions`].
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
    ) -> MsgSetUserGroupPermissions {
        MsgSetUserGroupPermissions {
            subspace_id,
            group_id,
            permissions: permissions.into_iter().map(Into::into).collect(),
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgDeleteUserGroup`].
    ///
    /// * `subspace_id` - Id of the subspace to which the group belongs.
    /// * `group_id` - Id of the group to delete.
    /// * `signer` - Address of who wants to delete the group.
    pub fn delete_user_group(subspace_id: u64, group_id: u32, signer: Addr) -> MsgDeleteUserGroup {
        MsgDeleteUserGroup {
            subspace_id,
            group_id,
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgAddUserToUserGroup`].
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
    ) -> MsgAddUserToUserGroup {
        MsgAddUserToUserGroup {
            subspace_id,
            group_id,
            user: user.into(),
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgRemoveUserFromUserGroup`].
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
    ) -> MsgRemoveUserFromUserGroup {
        MsgRemoveUserFromUserGroup {
            subspace_id,
            group_id,
            user: user.into(),
            signer: signer.into(),
        }
    }

    /// Creates a new instance of [`MsgSetUserPermissions`].
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
    ) -> MsgSetUserPermissions {
        MsgSetUserPermissions {
            subspace_id,
            section_id,
            user: user.into(),
            permissions: permissions.into_iter().map(Into::into).collect(),
            signer: signer.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_subspace() {
        let msg = SubspacesMsg::create_subspace(
            "test",
            "test",
            Addr::unchecked("owner"),
            Addr::unchecked("signer"),
        );
        let expected = MsgCreateSubspace {
            name: "test".into(),
            description: "test".into(),
            owner: "owner".into(),
            creator: "signer".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_edit_subspace() {
        let msg = SubspacesMsg::edit_subspace(
            42,
            "test",
            "test",
            Addr::unchecked("owner"),
            Addr::unchecked("signer"),
        );
        let expected = MsgEditSubspace {
            subspace_id: 42,
            name: "test".into(),
            description: "test".into(),
            owner: "owner".into(),
            signer: "signer".into(),
        };
        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_subspace() {
        let msg = SubspacesMsg::delete_subspace(1, Addr::unchecked("signer"));

        let expected = MsgDeleteSubspace {
            subspace_id: 1,
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_create_section() {
        let msg = SubspacesMsg::create_section(
            1,
            "test_section",
            Some("test description".into()),
            Some(1),
            Addr::unchecked("signer"),
        );

        let expected = MsgCreateSection {
            subspace_id: 1,
            name: "test_section".into(),
            description: "test description".into(),
            parent_id: 1,
            creator: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_edit_section() {
        let msg = SubspacesMsg::edit_section(
            1,
            1,
            Some("test_section".into()),
            Some("test description".into()),
            Addr::unchecked("signer"),
        );

        let expected = MsgEditSection {
            subspace_id: 1,
            section_id: 1,
            name: "test_section".into(),
            description: "test description".into(),
            editor: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_move_section() {
        let msg = SubspacesMsg::move_section(1, 1, 2, Addr::unchecked("signer"));

        let expected = MsgMoveSection {
            subspace_id: 1,
            section_id: 1,
            new_parent_id: 2,
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_section() {
        let msg = SubspacesMsg::delete_section(1, 1, Addr::unchecked("signer"));

        let expected = MsgDeleteSection {
            subspace_id: 1,
            section_id: 1,
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_create_user_group() {
        let msg = SubspacesMsg::create_user_group(
            1,
            Some(1),
            "test".into(),
            Some("test".into()),
            vec![],
            vec![],
            Addr::unchecked("signer"),
        );

        let expected = MsgCreateUserGroup {
            subspace_id: 1,
            section_id: 1,
            name: "test".into(),
            description: "test".into(),
            default_permissions: vec![],
            initial_members: vec![],
            creator: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_edit_user_group() {
        let msg = SubspacesMsg::edit_user_group(
            1,
            1,
            Some("test".into()),
            Some("test".into()),
            Addr::unchecked("signer"),
        );

        let expected = MsgEditUserGroup {
            subspace_id: 1,
            group_id: 1,
            name: "test".into(),
            description: "test".into(),
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_move_user_group() {
        let msg = SubspacesMsg::move_user_group(1, 1, 2, Addr::unchecked("signer"));

        let expected = MsgMoveUserGroup {
            subspace_id: 1,
            group_id: 1,
            new_section_id: 2,
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_set_user_group_permissions() {
        let msg = SubspacesMsg::set_user_group_permissions(
            1,
            1,
            vec![],
            Addr::unchecked("signer"),
        );

        let expected = MsgSetUserGroupPermissions {
            subspace_id: 1,
            group_id: 1,
            permissions: vec![],
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_delete_user_group() {
        let msg = SubspacesMsg::delete_user_group(1, 1, Addr::unchecked("signer"));

        let expected = MsgDeleteUserGroup {
            subspace_id: 1,
            group_id: 1,
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_add_user_to_user_group() {
        let msg = SubspacesMsg::add_user_to_user_group(
            1,
            1,
            Addr::unchecked("user"),
            Addr::unchecked("signer"),
        );

        let expected = MsgAddUserToUserGroup {
            subspace_id: 1,
            group_id: 1,
            user: "user".into(),
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_remove_user_to_user_group() {
        let msg = SubspacesMsg::remove_user_from_user_group(
            1,
            1,
            Addr::unchecked("user"),
            Addr::unchecked("signer"),
        );

        let expected = MsgRemoveUserFromUserGroup {
            subspace_id: 1,
            group_id: 1,
            user: "user".into(),
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_set_user_permissions() {
        let msg = SubspacesMsg::set_user_permissions(
            1,
            1,
            Addr::unchecked("user"),
            vec![],
            Addr::unchecked("signer"),
        );

        let expected = MsgSetUserPermissions {
            subspace_id: 1,
            section_id: 1,
            user: "user".into(),
            permissions: vec![],
            signer: "signer".into(),
        };

        assert_eq!(expected, msg)
    }
}
