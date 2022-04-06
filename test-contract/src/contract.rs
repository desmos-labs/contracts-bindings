use crate::error::ContractError;
use crate::msg::{
    AllClaimableTipsResponse, ClaimableTipsResponse, ExecuteMsg, InstantiateMsg, ProfileResponse,
    QueryMsg, UnclaimedTipsResponse,
};
use crate::state::{ADMIN, TIPS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::StdError::GenericErr;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdError, StdResult, Uint64,
};
use cw2::set_contract_version;
use desmos_bindings::profiles::msg::ProfilesMsg;
use desmos_bindings::profiles::querier::ProfilesQuerier;
use desmos_std::profiles::querier::ProfilesQuerier;
use desmos_std::types::PageRequest;
use std::collections::HashMap;
use std::vec::Vec;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:desmos-tip";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ADMIN.set(deps, Some(info.sender.clone()))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SaveProfile {
            dtag,
            bio,
            cover_picture,
            nickname,
            profile_picture,
            creator,
        } => Ok(Response::new().add_message(ProfilesMsg::SaveProfile {
            dtag,
            bio,
            cover_picture,
            nickname,
            profile_picture,
            creator: Addr::unchecked(creator),
        })),
        _ => Err(ContractError::NotSupported {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Profile { user } => {
            let profile_querier = ProfilesQuerier::new(&deps.querier);
            let profile_response = profile_querier.query_profile(Addr::unchecked(user))?;
            to_binary(&ProfileResponse {
                profile: profile_response.profile,
            })
        }
        _ => Err(StdError::generic_err("query request not supported")),
    }
}
