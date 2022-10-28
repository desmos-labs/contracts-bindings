//! Contains utilities,structs and enum to interact with the Desmos x/reactions module.

#[cfg(feature = "msg")]
pub mod msg;
#[cfg(feature = "query")]
pub mod querier;
pub mod types;

pub mod proto {
    pub use desmos_std::proto::desmos::reactions::v1::*;
}