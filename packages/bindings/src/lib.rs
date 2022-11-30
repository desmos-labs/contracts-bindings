#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
//! Crate that provides the bindings to interact with the Desmos blockchain custom modules from a CosmWasm
//! smart contract.

#[cfg(feature = "stargate")]
mod stargate;
#[cfg(feature = "legacy")]
mod legacy;

extern crate core;
#[cfg(feature = "iterators")]
pub mod iter;
#[cfg(feature = "stargate")]
pub use crate::stargate::*;
#[cfg(feature = "legacy")]
pub use crate::legacy::*;
