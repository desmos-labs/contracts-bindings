//! Contains the messages that can be sent to the chain to interact with the x/tokenfactory module.

use cosmwasm_std::{Addr, Coin};

use crate::cosmos_types::Metadata;
use crate::tokenfactory::types::*;

/// TokenfactoryMsg is the builder to generate Desmos x/tokenfactory messages.
pub struct TokenfactoryMsg {}

impl TokenfactoryMsg {
    pub fn create_denom(subspace_id: u64, sender: Addr, subdenom: &str) -> MsgCreateDenom {
        MsgCreateDenom {
            subspace_id,
            sender: sender.into(),
            subdenom: subdenom.into(),
        }
    }

    pub fn mint(subspace_id: u64, sender: Addr, amount: Coin) -> MsgMint {
        MsgMint {
            subspace_id,
            sender: sender.into(),
            amount: Some(amount.into()),
        }
    }

    pub fn burn(subspace_id: u64, sender: Addr, amount: Coin) -> MsgBurn {
        MsgBurn {
            subspace_id,
            sender: sender.into(),
            amount: Some(amount.into()),
        }
    }

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
