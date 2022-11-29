//! Contains utilities,structs and enum to interact with the Desmos x/relationships module.


pub mod msg;
pub mod querier;
pub mod proto {
    pub use desmos_std::proto::desmos::relationships::v1::*;
}
