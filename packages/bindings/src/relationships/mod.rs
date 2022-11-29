//! Contains utilities,structs and enum to interact with the Desmos x/relationships module.

#[cfg(not(target_arch = "wasm32"))]
pub mod mocks;
pub mod msg;
pub mod querier;
pub mod proto {
    pub use desmos_std::proto::desmos::relationships::v1::*;
}
