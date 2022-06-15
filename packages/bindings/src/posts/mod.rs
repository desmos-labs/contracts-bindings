//! Contains utilities,structs and enum to interact with the Desmos x/posts module.

pub mod models;
#[cfg(feature = "query")]
pub mod models_query;
#[cfg(feature = "msg")]
pub mod msg;
#[cfg(feature = "query")]
pub mod query;
