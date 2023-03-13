# Bindings test

Suite to test the interaction of the bindings with a real chain.  

To run the tests follow these instructions:
* Go inside the `contracts/test-contract` 
* Run `cargo optimize` to build the test contract
* Get the latest release of Desmos from [here](https://github.com/desmos-labs/desmos/releases) and place it inside the `scripts` directory  (**NOTE**: Be sure to download the correct one by checking at the compatibility table).
**NOTE**: The binary must have name `desmos`
* From the `scripts` directory launch the `spawn_test_chain.sh` script to launch a test chain
* From another terminal window, go this directory run `cargo run` to setup the chain with messages related to the contract
* Go inside the `scripts` directory and launch the `setup_chain.sh` script to prepare the chain for the tests
* From this directory run `cargo test --all-features -- --test-threads=1` to start the tests