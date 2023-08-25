//! Contains the messages that can be sent to the chain to interact with the x/tokenfactory module.

use cosmwasm_std::{Addr, Coin};

use crate::cosmos_types::Metadata;
use crate::tokenfactory::types::*;

/// TokenfactoryMsg is the builder to generate Desmos x/tokenfactory messages.
pub struct TokenfactoryMsg {}

impl TokenfactoryMsg {
    /// Creates a new instance of [`MsgCreateDenom`].
    ///
    /// * `subspace_id` - Id of the subspace which creates the denom.
    /// * `sender` - Address of user having the permission to manage subspace denoms.
    /// * `subdenom` - Subdenom name of the creating denom.
    pub fn create_denom(subspace_id: u64, sender: Addr, subdenom: &str) -> MsgCreateDenom {
        MsgCreateDenom {
            subspace_id,
            sender: sender.into(),
            subdenom: subdenom.into(),
        }
    }

    /// Creates a new instance of [`MsgMint`].
    ///
    /// * `subspace_id` - Id of the subspace which manages the denom.
    /// * `sender` - Address of user having the permission to manage subspace denoms.
    /// * `amount` - Amount of the minting subspace tokens.
    pub fn mint(subspace_id: u64, sender: Addr, amount: Coin) -> MsgMint {
        MsgMint {
            subspace_id,
            sender: sender.into(),
            amount: Some(amount.into()),
        }
    }

    /// Creates a new instance of [`MsgBurn`].
    /// 
    /// * `subspace_id` - Id of the subspace which manages the denom.
    /// * `sender` - Address of user having the permission to manage subspace denoms.
    /// * `amount` - Amount of the burning subspace tokens.
    pub fn burn(subspace_id: u64, sender: Addr, amount: Coin) -> MsgBurn {
        MsgBurn {
            subspace_id,
            sender: sender.into(),
            amount: Some(amount.into()),
        }
    }

    /// Creates a new instance of [`MsgSetDenomMetadata`].
    ///
    /// * `subspace_id` - Id of the subspace which manages the denom.
    /// * `sender` - Address of user having the permission to manage subspace denoms.
    /// * `metadata` - Metadata of the denom.
    pub fn set_denom_metadata(
        subspace_id: u64,
        sender: Addr,
        metadata: Metadata,
    ) -> MsgSetDenomMetadata {
        MsgSetDenomMetadata {
            subspace_id,
            sender: sender.into(),
            metadata: Some(metadata),
        }
    }
}
