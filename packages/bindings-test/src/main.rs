extern crate core;

mod chain_communication;

mod consts;
mod models;

#[cfg(feature = "legacy")]
mod legacy;

#[cfg(feature = "stargate")]
mod stargate;

fn main() {
    println!("Hello, world!");
}
