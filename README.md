# Contracts Bindings

<p align="center" width="100%">
    <img height="90" src="github/logo.svg" />
</p>

Desmos contracts bindings is a set of packages that allows you to create CosmWasm smart contracts that interacts directly with the Desmos Network custom modules.

![GitHub](https://img.shields.io/github/license/desmos-labs/desmos-bindings.svg) [![desmos-bindings on crates.io](https://img.shields.io/crates/v/desmos-bindings.svg)](https://crates.io/crates/desmos-bindings) [![codecov](https://codecov.io/gh/desmos-labs/desmos-bindings/branch/main/graph/badge.svg?token=TT3qCDd957)](https://codecov.io/gh/desmos-labs/desmos-bindings)

# Compatibility

Below the compatibility between the bindings and the Desmos chain

| Bindings Version | Desmos Version |
|------------------|----------------|
| **v3.0.x**       | **v6.1.x**     |
| **v2.0.x**       | **v5.0.x**     |
| **v1.2.x**       | **v4.6.x**     |
| **v1.1.x**       | **v4.3.x**     |
| **v1.0.0**       | **v4.1.x**     |

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
