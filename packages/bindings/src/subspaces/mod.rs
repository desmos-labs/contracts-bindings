//! Contains utilities,structs and enums to interact with the Desmos x/subspaces module.


pub mod msg;
pub mod querier;
pub mod types;
pub mod proto {
    pub use desmos_std::proto::desmos::subspaces::v3::*;
}
