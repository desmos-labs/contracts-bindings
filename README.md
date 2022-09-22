# Desmos Bindings

This repository contains the CosmWasm bindings packages that allows the interaction with the Desmos chain features from smart contracts.

![GitHub](https://img.shields.io/github/license/desmos-labs/desmos-bindings.svg) [![desmos-bindings on crates.io](https://img.shields.io/crates/v/desmos-bindings.svg)](https://crates.io/crates/desmos-bindings) [![codecov](https://codecov.io/gh/desmos-labs/desmos-bindings/branch/main/graph/badge.svg?token=TT3qCDd957)](https://codecov.io/gh/desmos-labs/desmos-bindings)

# Compatibility

Below the compatibility between the bindings and the Desmos chain

| Bindings Version | Desmos Version |
|---|---|
| **v1.1.x** | **v4.3.0** |
| **v1.0.0** | **v4.1.0** |

# Create a new contract
Assuming you have a recent version of rust and cargo (v1.58.1+) installed
(via [rustup](https://rustup.rs/)),
then the following instruction should provide you with a new template contract already set-up to interact with the bindings:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and [cargo-run-script](https://github.com/JoshMcguigan/cargo-run-script).
Unless you did that before, you can install them by running the following commands:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now you can use `cargo-generate` to set-up your new contract.
Go to the folder in which you want to place the project and run:

```sh
cargo generate --git https://github.com/desmos-labs/cw-template.git --name PROJECT_NAME
````
