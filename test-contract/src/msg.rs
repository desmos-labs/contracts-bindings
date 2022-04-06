use cosmwasm_std::{Addr, Coin};
use desmos_bindings::profiles::models_profile::Profile;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SaveProfile {
        dtag: String,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
        creator: String,
    },
    DeleteProfile {
        creator: String,
    },
    RequestDtagTransfer {
        receiver: String,
        sender: String,
    },
    AcceptDtagTransferRequest {
        new_dtag: String,
        sender: String,
        receiver: String,
    },
    RefuseDtagTransferRequest {
        sender: String,
        receiver: String,
    },
    CancelDtagTransferRequest {
        receiver: String,
        sender: String,
    },
    LinkChainAccount {},
    LinkApplication {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Profile {
        user: String,
    },
    IncomingDtagTransferRequest {
        receiver: String,
    },
    ChainLink {
        user: Option<String>,
        chain_name: Option<String>,
        target: Option<String>,
    },
    AppLink {
        user: Option<String>,
        application: Option<String>,
        username: Option<String>,
    },
    ApplicationLinkByChainID {
        client_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProfileResponse {
    pub profile: Profile,
}
