//! Contains the messages that can be sent to the chain to interact with the x/reactions module.

use cosmwasm_std::Addr;
use crate::reactions::types::ReactionValue;
use crate::reactions::proto::*;

/// Represents the messages to interact with the reactions module.
pub struct ReactionsMsgBuilder {}

impl ReactionsMsgBuilder {
    /// Creates a new instance of [`ReactionsMsg::AddReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the post to react to is.
    /// * `post_id` - Id of the post to react to.
    /// * `value` - Value of the reaction.
    /// * `user` - User reacting to the post.
    pub fn add_reaction(subspace_id: u64, post_id: u64, value: ReactionValue, user: Addr) -> MsgAddReaction {
        MsgAddReaction {
            subspace_id,
            post_id,
            value: Some(value.into()),
            user: user.into(),
        }
    }
    /// Creates a new instance of [`ReactionsMsg::RemoveReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the reaction to remove is.
    /// * `post_id` - Id of the post from which to remove the reaction.
    /// * `reaction_id` - Id of the reaction to be removed.
    /// * `user` - User removing the reaction.
    pub fn remove_reaction(subspace_id: u64, post_id: u64, reaction_id: u32, user: Addr) -> MsgRemoveReaction {
        MsgRemoveReaction {
            subspace_id,
            post_id,
            reaction_id,
            user: user.into(),
        }
    }
    /// Creates a new instance of [`ReactionsMsg::AddRegisteredReaction`].
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
    /// Creates a new instance of [`ReactionsMsg::EditRegisteredReaction`].
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
    /// Creates a new instance of [`ReactionsMsg::RemoveRegisteredReaction`].
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
    /// Creates a new instance of [`ReactionsMsg::SetReactionsParams`].
    ///
    /// * `subspace_id` - Id of the subspace for which to set the params.
    /// * `registered_reaction` - Params related to [`ReactionValue::Registered`](crate::reactions::models::ReactionValue::Registered) reactions.
    /// * `value` - Params related to [`ReactionValue::FreeText`](crate::reactions::models::ReactionValue::FreeText) reactions.
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
mod tests {}
