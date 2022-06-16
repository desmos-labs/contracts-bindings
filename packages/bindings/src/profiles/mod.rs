//! Contains utilities,structs and enum to interact with the Desmos x/profiles module.

pub mod models_app_links;
pub mod models_chain_links;
pub mod models_dtag_requests;
pub mod models_profile;
pub mod models_query;
pub mod msg;
#[cfg(feature = "query")]
pub mod querier;
#[cfg(feature = "query")]
pub mod query;

#[cfg(all(not(target_arch = "wasm32"), feature = "query"))]
pub mod mocks;
