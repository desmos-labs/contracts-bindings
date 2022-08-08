//! Contains utilities,structs and enum to interact with the Desmos x/relationships module.

#[cfg(not(target_arch = "wasm32"))]
pub mod mocks;

pub mod models;
pub mod models_query;
pub mod msg;
#[cfg(feature = "query")]
pub mod querier;
#[cfg(feature = "query")]
pub mod query;
