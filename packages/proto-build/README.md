# Desmos proto build

It is a program to generate the standard library for desmos bindings from desmos proto files.

The feature is depend on `buf`, please install it first by following command:
```bash
GO111MODULE=on go install github.com/bufbuild/buf/cmd/buf@v1.17.0
```

To generate std library, run:
```bash
cargo run
```

To update the desmos dependency then generate standard library, run:
```bash
cargo run -- --update-deps
``` 