//! Contains utilities,structs and enums to interact with the Desmos x/subspaces module.

#[cfg(not(target_arch = "wasm32"))]
pub mod mocks;
pub mod models;
pub mod models_query;
pub mod msg;
pub mod querier;
pub mod query;
