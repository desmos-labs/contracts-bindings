use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ProfileResponse, QueryMsg};
use crate::state::ADMIN;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use desmos_bindings::msg::DesmosMsg;
use desmos_bindings::profiles::msg::ProfilesMsg;
use desmos_bindings::profiles::querier::ProfilesQuerier;
use std::ops::Deref;

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
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    match msg {
        ExecuteMsg::SaveProfile {
            dtag,
            bio,
            cover_picture,
            nickname,
            profile_picture,
        } => Ok(Response::new().add_attribute("test", "test").add_message(
            ProfilesMsg::SaveProfile {
                dtag,
                bio,
                cover_picture,
                nickname,
                profile_picture,
                creator: env.contract.address,
            },
        )),
        ExecuteMsg::DeleteProfile {} => {
            Ok(Response::new().add_message(ProfilesMsg::delete_profile(env.contract.address)))
        }
        _ => Err(ContractError::NotSupported {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Profile { user } => {
            let profile_querier = ProfilesQuerier::new(deps.querier.deref());
            let profile_response = profile_querier.query_profile(Addr::unchecked(user))?;
            to_binary(&ProfileResponse {
                profile: profile_response.profile,
            })
        }
        _ => Err(StdError::generic_err("query request not supported")),
    }
}
