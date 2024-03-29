//! Contains utilities, structs and enum to interact with the Desmos x/reports module.

#[cfg(not(target_arch = "wasm32"))]
pub mod mocks;

pub mod msg;
pub mod querier;
pub mod types;
