//! Contains utilities,structs and enums to interact with the Desmos x/subspaces module.

#[cfg(feature = "msg")]
pub mod msg;
#[cfg(feature = "query")]
pub mod querier;
#[cfg(feature = "query")]

pub mod types;

pub mod proto {
    pub use desmos_std::proto::desmos::subspaces::v3::*;
}
