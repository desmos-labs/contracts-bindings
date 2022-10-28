pub enum Permission {
    /// Allows to change the information of the subspace.
    EditSubspace,
    /// Allows users to delete the subspace.
    DeleteSubspace,
    /// Allows users to manage a subspace sections.
    ManageSections,
    /// Allows users to manage user groups and members.
    ManageGroups,
    /// Allows users to set other users' permissions (except [`Permission::`SetPermissions`]).
    /// This includes managing user groups and the associated permissions.
    SetPermissions,
    /// Allows to do everything.
    /// This should usually be reserved only to the owner (which has it by default).
    Everything,
    /// Identifies users that can create content inside the subspace.
    Write,
    /// Allows users to interact with content inside the subspace (eg. polls).
    InteractWithContent,
    /// Allows users to edit their own content inside the subspace.
    EditOwnContent,
    /// Allows users to moderate other user's content.
    ModerateContent,
}

impl Into<String> for Permission {
    fn into(self) -> String {
        match self {
            Permission::EditSubspace => "EDIT_SUBSPACE".into(),
            Permission::DeleteSubspace => "DELETE_SUBSPACE".into(),
            Permission::ManageSections => "MANAGE_SECTIONS".into(),
            Permission::ManageGroups => "MANAGE_GROUPS".into(),
            Permission::SetPermissions => "SET_PERMISSIONS".into(),
            Permission::Everything => "EVERYTHING".into(),
            Permission::Write => "WRITE_CONTENT".into(),
            Permission::InteractWithContent => "INTERACT_WITH_CONTENT".into(),
            Permission::EditOwnContent => "EDIT_OWN_CONTENT".into(),
            Permission::ModerateContent => "MODERATE_CONTENT".into(),
        }
    }
}