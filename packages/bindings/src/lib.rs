#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
//! Crate that provides the bindings to interact with the Desmos blockchain custom modules from a CosmWasm
//! smart contract.

extern crate core;
#[cfg(feature = "iterators")]
pub mod iter;
#[cfg(all(not(target_arch = "wasm32"), feature = "mocks"))]
pub mod mocks;
#[cfg(feature = "msg")]
pub mod msg;
#[cfg(feature = "posts")]
pub mod posts;
#[cfg(feature = "profiles")]
pub mod profiles;
#[cfg(feature = "query")]
pub mod query;
#[cfg(feature = "reactions")]
pub mod reactions;
#[cfg(feature = "relationships")]
pub mod relationships;
#[cfg(feature = "reports")]
pub mod reports;
#[cfg(feature = "subspaces")]
pub mod subspaces;
pub mod types;
