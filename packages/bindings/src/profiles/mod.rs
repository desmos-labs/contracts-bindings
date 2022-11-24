//! Contains utilities,structs and enum to interact with the Desmos x/profiles module.

#[cfg(not(target_arch = "wasm32"))]
pub mod mocks;
pub mod models_app_links;
pub mod models_chain_links;
pub mod models_dtag_requests;
pub mod models_profile;
pub mod models_query;
pub mod msg;
pub mod querier;
pub mod query;
