#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
//! Crate that provides the bindings to interact with the Desmos blockchain custom modules from a CosmWasm
//! smart contract.

#[cfg(feature = "legacy")]
mod legacy;
#[cfg(feature = "stargate")]
mod stargate;

extern crate core;
#[cfg(feature = "iterators")]
pub mod iter;
#[cfg(feature = "legacy")]
pub use crate::legacy::*;
#[cfg(feature = "stargate")]
pub use crate::stargate::*;
