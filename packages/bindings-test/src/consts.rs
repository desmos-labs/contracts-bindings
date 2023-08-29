use cosmwasm_std::{Uint64, Coin, Uint128};

pub const GAS: Uint64 = Uint64::new(300000);

pub const USER1_KEY: &str = "user1";
pub const USER1_ADDRESS: &str = "desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3";

pub const USER2_KEY: &str = "user2";
pub const USER2_ADDRESS: &str = "desmos1ptvq7l4jt7n9sc3fky22mfvc6waf2jd8nuc0jv";

pub const TEST_SUBSPACE: u64 = 1;
pub const TEST_SUBSPACE_USER_GROUP: u32 = 1;
pub const TEST_SUBSPACE_EDITABLE_POST_ID: u64 = 1;
pub const TEST_SUBSPACE_DELETABLE_POST_ID: u64 = 2;
pub const TEST_DELETABLE_ATTACHMENT_ID: u32 = 2;
pub const TEST_POLL_ID: u32 = 1;
pub const TEST_REASON_ID: u32 = 1;
pub const TEST_DELETABLE_REASON_ID: u32 = 2;
pub const TEST_REPORT_ID_WITH_USER_TARGET: u64 = 1;
pub const TEST_DELETABLE_REPORT_ID: u64 = 3;

pub const TEST_EDITABLE_REGISTERED_REACTION_ID: u32 = 1;
pub const TEST_DELETABLE_REGISTERED_REACTION_ID: u32 = 2;
pub const TEST_REACTIONS_POST_ID: u64 = 3;
pub const TEST_POST_REGISTERED_REACTION_ID: u32 = 1;
pub const TEST_POST_FREE_TEXT_REACTION_ID: u32 = 2;
pub const TEST_POST_DELETABLE_REACTION_ID: u32 = 3;

pub const TEST_CREATION_DENOM_FEES_AMOUNT: u128 = 10000000000;
pub const TEST_CREATION_DENOM_FEES_DENOM: &str = "stake";
