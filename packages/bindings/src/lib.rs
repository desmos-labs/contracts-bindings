#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
//! Crate that provides the bindings to interact with the Desmos blockchain custom modules from a CosmWasm
//! smart contract.

extern crate core;
#[cfg(feature = "iterators")]
pub mod iter;
#[cfg(feature = "posts")]
pub mod posts;
#[cfg(feature = "profiles")]
pub mod profiles;
#[cfg(feature = "reactions")]
pub mod reactions;
#[cfg(feature = "relationships")]
pub mod relationships;
#[cfg(feature = "reports")]
pub mod reports;
#[cfg(feature = "subspaces")]
pub mod subspaces;

pub mod types;
