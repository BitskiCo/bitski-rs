# Bitski Rust SDK

## Install

```bash
cargo add bitski bitski-provider
```

## Connect via OAuth

You can auth via your Bitski credentials, which can be generated at
`developer.bitski.com`, and return a web3 provider.

If you don't need to send authenticated requests, e.g. for read-only data, you
can use `BITSKI_API_KEY=local`.

```rust,ignore
use bitski::Bitski;
use tokio;
use web3::Transport;

#[tokio::main]
async fn main() {
    let bitski = Bitski::from_env().expect("Could not initialize");
    let network = "mainnet";
    let provider = bitski
        .get_provider(network)
        .expect("Could not get provider");

    let method = "eth_getTransactionCount";
    let params =
        serde_json::from_str(r#"["0x457044DFF5886a9eb9365015704e1b747F384194", "latest"]"#)
            .expect("Invalid params");
    let result = provider.execute(method, params).await;

    match result {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```
