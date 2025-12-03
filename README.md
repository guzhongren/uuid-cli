# uuidv7

Small CLI to generate UUID v7 values.

Build

```sh
cargo build --release
```

Examples

```sh
# generate one
cargo run --release -- --count 1

# generate 5, uppercase, no hyphens
cargo run --release -- -n 5 --upper --no-hyphen
```
