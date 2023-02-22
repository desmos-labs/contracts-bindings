//! Contains the messages that can be sent to the chain to interact with the x/reactions module.

use crate::stargate::reactions::types::*;
use cosmwasm_std::Addr;

/// ReactionsMsgBuilder is the builder to generate Desmos x/reactions messages.
pub struct ReactionsMsgBuilder {}

impl ReactionsMsgBuilder {
    /// Creates a new instance of [`MsgAddReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the post to react to is.
    /// * `post_id` - Id of the post to react to.
    /// * `value` - Value of the reaction.
    /// * `user` - User reacting to the post.
    pub fn add_reaction(
        subspace_id: u64,
        post_id: u64,
        value: ReactionValue,
        user: Addr,
    ) -> MsgAddReaction {
        MsgAddReaction {
            subspace_id,
            post_id,
            value: Some(value.into()),
            user: user.into(),
        }
    }
    /// Creates a new instance of [`MsgRemoveReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the reaction to remove is.
    /// * `post_id` - Id of the post from which to remove the reaction.
    /// * `reaction_id` - Id of the reaction to be removed.
    /// * `user` - User removing the reaction.
    pub fn remove_reaction(
        subspace_id: u64,
        post_id: u64,
        reaction_id: u32,
        user: Addr,
    ) -> MsgRemoveReaction {
        MsgRemoveReaction {
            subspace_id,
            post_id,
            reaction_id,
            user: user.into(),
        }
    }
    /// Creates a new instance of [`MsgAddRegisteredReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which this reaction should be registered.
    /// * `shorthand_code` - Shorthand code of the reaction.
    /// * `display_value` - Display value of the reaction.
    /// * `user` - User adding the supported reaction.
    pub fn add_registered_reaction(
        subspace_id: u64,
        shorthand_code: &str,
        display_value: &str,
        user: Addr,
    ) -> MsgAddRegisteredReaction {
        MsgAddRegisteredReaction {
            subspace_id,
            shorthand_code: shorthand_code.into(),
            display_value: display_value.into(),
            user: user.into(),
        }
    }
    /// Creates a new instance of [`MsgEditRegisteredReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the reaction to edit is.
    /// * `registered_reaction_id` - Id of the registered reaction to edit.
    /// * `shorthand_code` - New shorthand code to be set.
    /// * `display_value` - Display value to be set.
    /// * `user` - User editing the registered reaction.
    pub fn edit_registered_reaction(
        subspace_id: u64,
        registered_reaction_id: u32,
        shorthand_code: &str,
        display_value: &str,
        user: Addr,
    ) -> MsgEditRegisteredReaction {
        MsgEditRegisteredReaction {
            subspace_id,
            registered_reaction_id,
            shorthand_code: shorthand_code.into(),
            display_value: display_value.into(),
            user: user.into(),
        }
    }
    /// Creates a new instance of [`MsgRemoveRegisteredReaction`].
    ///
    /// * `subspace_id` - Id of the registered reaction to be removed.
    /// * `registered_reaction_id` - Id of the registered reaction to be removed.
    /// * `user` - User removing the registered reaction.
    pub fn remove_registered_reaction(
        subspace_id: u64,
        registered_reaction_id: u32,
        user: Addr,
    ) -> MsgRemoveRegisteredReaction {
        MsgRemoveRegisteredReaction {
            subspace_id,
            registered_reaction_id,
            user: user.into(),
        }
    }
    /// Creates a new instance of [`MsgSetReactionsParams`].
    ///
    /// * `subspace_id` - Id of the subspace for which to set the params.
    /// * `registered_reaction` - Params related to [`FreeTextValue`].
    /// * `free_text` - Params related to [`RegisteredReactionValue`].
    /// * `user` - User setting the params.
    pub fn set_reactions_params(
        subspace_id: u64,
        registered_reaction: Option<RegisteredReactionValueParams>,
        free_text: Option<FreeTextValueParams>,
        user: Addr,
    ) -> MsgSetReactionsParams {
        MsgSetReactionsParams {
            subspace_id: subspace_id.into(),
            registered_reaction,
            free_text,
            user: user.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_reaction() {
        let msg = ReactionsMsgBuilder::add_reaction(
            1,
            1,
            ReactionValue::Registered(RegisteredReactionValue {
                registered_reaction_id: 1,
            })
            .into(),
            Addr::unchecked("user"),
        );

        let expected = MsgAddReaction {
            subspace_id: 1,
            post_id: 1,
            value: Some(
                ReactionValue::Registered(RegisteredReactionValue {
                    registered_reaction_id: 1,
                })
                .into(),
            ),
            user: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_remove_reaction() {
        let msg = ReactionsMsgBuilder::remove_reaction(1, 1, 1, Addr::unchecked("user"));

        let expected = MsgRemoveReaction {
            subspace_id: 1,
            post_id: 1,
            reaction_id: 1,
            user: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_add_registered_reaction() {
        let msg = ReactionsMsgBuilder::add_registered_reaction(
            1,
            "test_code",
            "test_value",
            Addr::unchecked("user"),
        );

        let expected = MsgAddRegisteredReaction {
            subspace_id: 1,
            shorthand_code: "test_code".into(),
            display_value: "test_value".into(),
            user: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_edit_registered_reaction() {
        let msg = ReactionsMsgBuilder::edit_registered_reaction(
            1,
            1,
            "test_code",
            "test_value",
            Addr::unchecked("user"),
        );

        let expected = MsgEditRegisteredReaction {
            subspace_id: 1,
            registered_reaction_id: 1,
            shorthand_code: "test_code".into(),
            display_value: "test_value".into(),
            user: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_remove_registered_reaction() {
        let msg = ReactionsMsgBuilder::remove_registered_reaction(1, 1, Addr::unchecked("user"));

        let expected = MsgRemoveRegisteredReaction {
            subspace_id: 1,
            registered_reaction_id: 1,
            user: "user".into(),
        };

        assert_eq!(expected, msg)
    }

    #[test]
    fn test_set_reactions_params() {
        let msg = ReactionsMsgBuilder::set_reactions_params(
            1,
            Some(RegisteredReactionValueParams { enabled: true }),
            Some(FreeTextValueParams {
                enabled: true,
                max_length: 100,
                reg_ex: "".into(),
            }),
            Addr::unchecked("user"),
        );

        let expected = MsgSetReactionsParams {
            subspace_id: 1,
            registered_reaction: Some(RegisteredReactionValueParams { enabled: true }),
            free_text: Some(FreeTextValueParams {
                enabled: true,
                max_length: 100,
                reg_ex: "".into(),
            }),
            user: "user".into(),
        };

        assert_eq!(expected, msg)
    }
}
