# Bitski Rust SDK

## Install

```toml
bitski = { git = "https://github.com/BitskiCo/bitski-rs" }
bitski-provider = { git = "https://github.com/BitskiCo/bitski-rs" }
```

## Connect via OAuth

Can auth via your Bitski credentials and return a web3 provider.

```rust
fn main() {
    let bitski = bitski::Bitski::from_env().expect("Could not initialize");
    let network = "mainnet";
    let provider = bitski
        .get_provider(network)
        .expect("Could not get provider");
}
```

## Local node with Anvil

If you would like to use a local node, i.e. during tests, first install
[Anvil][anvil]. Anvil is an Ethereum node written in Rust that can be used for
testing RPC calls, contract interactions, etc.

Then, run a local instance of Anvil on `localhost:8545`:

```sh
anvil
```

In `Cargo.toml`, enable the `local` feature:

```toml
bitski = { git = "https://github.com/BitskiCo/bitski-rs", features = ["local"] }
```

Finally, write your tests:

```rust
fn main() {
    // Initialize the Anvil provider
    let bitski = bitski::Bitski::new_local_mode(None);

    // Get a web3 provider
    let web3 = bitski
        .into_inner()
        .get_web3_test_mode("anvil")
        .expect("Could not get web3 provider");
}
```

[anvil]: https://github.com/foundry-rs/foundry/tree/master/anvil
