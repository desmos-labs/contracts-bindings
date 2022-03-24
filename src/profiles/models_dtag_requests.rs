use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represent a DTag transfer request between two users.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DtagTransferRequest {
    /// Value of the DTag that should be transferred from
    /// the receiver of the request to the sender.
    pub dtag_to_trade: String,
    /// The address of the account that sent the request.
    pub sender: Addr,
    /// The receiver of the request that, if accepted, will give to the sender their DTag.
    pub receiver: Addr,
}
