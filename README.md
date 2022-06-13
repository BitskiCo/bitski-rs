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

If you would like to use a local node, i.e. during tests, the `local` feature
needs to be enabled:

```toml
bitski = { git = "https://github.com/BitskiCo/bitski-rs", features = ["local"] }
```

This installs [Anvil](https://github.com/foundry-rs/foundry/tree/master/anvil),
an Ethereum node written in Rust that can be used for testing RPC calls, 
contract interactions, etc. It can be run directly in your Rust program, so a
CLI program like Ganache or Hardhat Network aren't needed.

```rust
fn main() {
    // Start an Anvil node at `localhost:8545` and initialize a provider
    let _anvil = bitski::Anvil::new(None).await;
    let bitski = bitski::Bitski::new_local_mode(None);

    // Or, specify a port to run on
    // let anvil = bitski::Anvil::new(Some(8888)).await;
    // let bitski = bitski::Bitski::new_local_mode(Some(anvil.rpc_url.clone()));

    // Get a web3 provider
    let web3 = bitski
        .into_inner()
        .get_web3_test_mode("anvil")
        .expect("Could not get web3 provider");
}
```