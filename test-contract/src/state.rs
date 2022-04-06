use cosmwasm_std::Coin;
use cw_controllers::Admin;
use cw_storage_plus::Map;

// Contains the tips not distributed.
// The key of the map is a tuple where the first item is the
// application and the second item the username, for example a tip to DesmosNetwork on twitter
// will be (twitter, DesmosNetwork).
pub const TIPS: Map<(&str, &str), Vec<Coin>> = Map::new("tips");
pub const ADMIN: Admin = Admin::new("admin");
