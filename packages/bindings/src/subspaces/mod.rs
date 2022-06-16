//! Contains utilities,structs and enums to interact with the Desmos x/subspaces module.

#[cfg(all(not(target_arch = "wasm32"), feature = "query"))]
pub mod mocks;

pub mod models;
pub mod msg;
#[cfg(feature = "query")]
pub mod querier;
#[cfg(feature = "query")]
pub mod query;

pub mod query_types;
