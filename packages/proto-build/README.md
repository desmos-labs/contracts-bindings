# Desmos proto build

It is a program to generate the standard library for desmos bindings from desmos proto files.

To generate std library, run:
```bash
cargo run
```

To update the desmos dependency then generate standard library, run:
```bash
cargo run -- --update-deps
``` 