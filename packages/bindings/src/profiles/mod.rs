//! Contains utilities,structs and enum to interact with the Desmos x/profiles module.

#[cfg(feature = "msg")]
pub mod msg;
#[cfg(feature = "query")]
pub mod querier;

pub mod types;

pub use proto::*;

mod proto {
    pub use desmos_std::proto::desmos::profiles::v3::*;
}