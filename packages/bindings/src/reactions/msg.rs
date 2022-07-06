//! Contains the messages that can be sent to the chain to interact with the x/reactions module.

use crate::reactions::models::{
    FreeTextValueParams, RawReactionValue, ReactionValue, RegisteredReactionValueParams,
};

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the messages to interact with the reactions module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsMsg {
    /// Add a reaction to a post.
    AddReaction {
        /// Id of the subspace inside which the post to react to is.
        subspace_id: Uint64,
        /// Id of the post to react to.
        post_id: Uint64,
        /// Value of the reaction.
        value: RawReactionValue,
        /// User reacting to the post.
        user: Addr,
    },
    /// Remove a reaction from a post.
    RemoveReaction {
        /// Id of the subspace inside which the reaction to remove is.
        subspace_id: Uint64,
        /// Id of the post from which to remove the reaction.
        post_id: Uint64,
        /// Id of the reaction to be removed.
        reaction_id: u32,
        /// User removing the reaction.
        user: Addr,
    },
    /// Register a new supported reaction to the subspace.
    AddRegisteredReaction {
        /// Id of the subspace inside which this reaction should be registered.
        subspace_id: Uint64,
        /// Shorthand code of the reaction.
        shorthand_code: String,
        /// Display value of the reaction.
        display_value: String,
        /// User adding the supported reaction.
        user: Addr,
    },
    /// Edit a registered reaction inside the subspace.
    EditRegisteredReaction {
        /// Id of the subspace inside which the reaction to edit is.
        subspace_id: Uint64,
        /// Id of the registered reaction to edit.
        registered_reaction_id: u32,
        /// New shorthand code to be set.
        shorthand_code: String,
        /// Display value to be set.
        display_value: String,
        /// User editing the registered reaction.
        user: Addr,
    },
    /// Remove a registered reaction from the subspace.
    RemoveRegisteredReaction {
        /// Id of the subspace from which to remove the registered reaction.
        subspace_id: Uint64,
        /// Id of the registered reaction to be removed.
        registered_reaction_id: u32,
        /// User removing the registered reaction.
        user: Addr,
    },
    /// Set reactions parameters for the subspace.
    SetReactionsParams {
        /// Id of the subspace for which to set the params.
        subspace_id: Uint64,
        /// Params related to [`RegisteredReactionValue`](crate::reactions::models::RegisteredReactionValue) reactions.
        registered_reaction: RegisteredReactionValueParams,
        /// Params related to [`FreeTextValue`](crate::reactions::models::FreeTextValue) reactions.
        free_text: FreeTextValueParams,
        /// User setting the params.
        user: Addr,
    },
}

impl ReactionsMsg {
    /// Creates a new instance of [`ReactionsMsg::AddReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the post to react to is.
    /// * `post_id` - Id of the post to react to.
    /// * `value` - Value of the reaction.
    /// * `user` - User reacting to the post.
    pub fn add_reaction(subspace_id: u64, post_id: u64, value: ReactionValue, user: Addr) -> Self {
        Self::AddReaction {
            subspace_id: subspace_id.into(),
            post_id: post_id.into(),
            value: value.into(),
            user,
        }
    }
    /// Creates a new instance of [`ReactionsMsg::RemoveReaction`].
    ///
    /// * `subspace_id` - Id of the subspace inside which the reaction to remove is.
    /// * `post_id` - Id of the post from which to remove the reaction.
    /// * `reaction_id` - Id of the reaction to be removed.
    /// * `user` - User removing the reaction.
    pub fn remove_reaction(subspace_id: u64, post_id: u64, reaction_id: u32, user: Addr) -> Self {
        Self::RemoveReaction {
            subspace_id: subspace_id.into(),
            post_id: post_id.into(),
            reaction_id,
            user,
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
    ) -> Self {
        Self::AddRegisteredReaction {
            subspace_id: subspace_id.into(),
            shorthand_code: shorthand_code.to_string(),
            display_value: display_value.to_string(),
            user,
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
    ) -> Self {
        Self::EditRegisteredReaction {
            subspace_id: subspace_id.into(),
            registered_reaction_id,
            shorthand_code: shorthand_code.to_string(),
            display_value: display_value.to_string(),
            user,
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
    ) -> Self {
        Self::RemoveRegisteredReaction {
            subspace_id: subspace_id.into(),
            registered_reaction_id,
            user,
        }
    }
    /// Creates a new instance of [`ReactionsMsg::SetReactionsParams`].
    ///
    /// * `subspace_id` - Id of the subspace for which to set the params.
    /// * `registered_reaction` - Params related to [`RegisteredReactionValue`](crate::reactions::models::RegisteredReactionValue) reactions.
    /// * `value` - Params related to [`FreeTextValue`](crate::reactions::models::FreeTextValue) reactions.
    /// * `user` - User setting the params.
    pub fn set_reactions_params(
        subspace_id: u64,
        registered_reaction: RegisteredReactionValueParams,
        free_text: FreeTextValueParams,
        user: Addr,
    ) -> Self {
        Self::SetReactionsParams {
            subspace_id: subspace_id.into(),
            registered_reaction,
            free_text,
            user,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_reaction() {
        let msg = ReactionsMsg::add_reaction(
            1,
            1,
            ReactionValue::Registered {
                registered_reaction_id: 1,
            },
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReactionsMsg::AddReaction {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            value: ReactionValue::Registered {
                registered_reaction_id: 1,
            }
            .into(),
            user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_remove_reaction() {
        let msg = ReactionsMsg::remove_reaction(
            1,
            1,
            1,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReactionsMsg::RemoveReaction {
            subspace_id: Uint64::new(1),
            post_id: Uint64::new(1),
            reaction_id: 1,
            user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_add_registered_reaction() {
        let msg = ReactionsMsg::add_registered_reaction(
            1,
            "test_code",
            "test_value",
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReactionsMsg::AddRegisteredReaction {
            subspace_id: Uint64::new(1),
            shorthand_code: "test_code".to_string(),
            display_value: "test_value".to_string(),
            user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_edit_registered_reaction() {
        let msg = ReactionsMsg::edit_registered_reaction(
            1,
            1,
            "test_code",
            "test_value",
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReactionsMsg::EditRegisteredReaction {
            subspace_id: Uint64::new(1),
            registered_reaction_id: 1,
            shorthand_code: "test_code".to_string(),
            display_value: "test_value".to_string(),
            user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_remove_registered_reaction() {
        let msg = ReactionsMsg::remove_registered_reaction(
            1,
            1,
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReactionsMsg::RemoveRegisteredReaction {
            subspace_id: Uint64::new(1),
            registered_reaction_id: 1,
            user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_set_reactions_params() {
        let msg = ReactionsMsg::set_reactions_params(
            1,
            RegisteredReactionValueParams { enabled: true },
            FreeTextValueParams {
                enabled: true,
                max_length: 100,
                reg_ex: "".to_string(),
            },
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        );
        let expected = ReactionsMsg::SetReactionsParams {
            subspace_id: Uint64::new(1),
            registered_reaction: RegisteredReactionValueParams { enabled: true },
            free_text: FreeTextValueParams {
                enabled: true,
                max_length: 100,
                reg_ex: "".to_string(),
            },
            user: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        };
        assert_eq!(msg, expected)
    }
}
