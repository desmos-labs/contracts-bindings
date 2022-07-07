# Desmos Bindings

This repository contains the CosmWasm bindings packages that allows the interaction with the Desmos chain features from smart contracts.


# Compatibility

Below the compatibility between the bindings and the Desmos chain

| Bindings Version | Desmos Version |
|---|---|
| **v1.0.0** | **v4.1.0** |

# Create a new contract
Assuming you have a recent version of rust and cargo (v1.58.1+) installed
(via [rustup](https://rustup.rs/)),
then the following instruction should provide you with a new template contract already set-up to interact with the bindings:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and [cargo-run-script](https://github.com/JoshMcguigan/cargo-run-script).
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

```sh
cargo generate --git https://github.com/desmos-labs/cw-template.git --name PROJECT_NAME
````
