# rust-cueball-dns-resolver

This is a rust implementation of a resolver or Joyent's
[cueball](https://github.com/joyent/rust-cueball) connection manager.

This crate includes:

* client library interface

**Note: This crate and its interfaces are unstable and will likely change underneath you without notice.**

# Build
```
cargo build
```

# Development
## Testing
```
cargo test
```

or

```
cargo test -- --nocapture
```

## Committing
Before commit, ensure that the following command is run:
```
cargo fmt
```
