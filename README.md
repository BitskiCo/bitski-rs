# Bitski Rust SDK

Can auth via Bitski and return a web3 provider.

```rust
let bitski = Bitski::from_env().expect("Could not initialize");
let provider = bitski
    .get_provider(network.as_ref())
    .expect("Could not get provider");
```