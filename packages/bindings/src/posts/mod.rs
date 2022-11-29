//! Contains utilities,structs and enum to interact with the Desmos x/posts module.

#[cfg(all(not(target_arch = "wasm32"), feature = "mocks"))]
pub mod mocks;
pub mod msg;
pub mod querier;
pub mod types;
pub mod proto {
    pub use desmos_std::proto::desmos::posts::v2::*;
}
