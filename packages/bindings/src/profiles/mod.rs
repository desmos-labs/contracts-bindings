//! Contains utilities,structs and enum to interact with the Desmos x/profiles module.

#[cfg(all(not(target_arch = "wasm32"), feature = "mocks"))]
pub mod mocks;
#[cfg(feature = "msg")]
pub mod msg;
#[cfg(feature = "query")]
pub mod querier;

pub mod types;

pub mod proto {
    pub use desmos_std::proto::desmos::profiles::v3::*;
}
