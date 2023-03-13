extern crate core;

mod chain_communication;

mod consts;
mod models;
mod setup;

mod posts;
mod profiles;
mod reactions;
mod relationships;
mod reports;
mod subspaces;

fn main() {
    println!("Setup contract state start!");
    setup::setup();
    println!("Setup contract state finished!");
}
