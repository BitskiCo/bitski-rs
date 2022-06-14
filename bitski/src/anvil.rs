use anvil::eth::EthApi;
use anvil::NodeConfig;
use anyhow::anyhow;
use std::str::FromStr;
use web3::types::U256;

static DEFAULT_PORT: u16 = 8545;

pub struct Anvil {
    /// Anvil API object to send commands to the node with.
    pub api: EthApi,
    /// The RPC url to connect to the node with.
    pub rpc_url: String,
}

impl Anvil {
    /// Create a new Anvil node. If a port is specified the node will be available at
    /// `localhost:<port>`, otherwise the default port of `8545` will be used.
    pub async fn new(port: Option<u16>) -> Self {
        let port = if let Some(port) = port {
            port
        } else {
            DEFAULT_PORT
        };
        let config = NodeConfig {
            port,
            enable_tracing: false,
            ..Default::default()
        };

        let (api, handle) = anvil::spawn(config).await;
        Anvil {
            api,
            rpc_url: handle.http_endpoint(),
        }
    }

    /// Sets the miniumum gas price for the node.
    pub async fn set_min_gas_price(&self, price: U256) -> Result<(), anyhow::Error> {
        // convert from the web3 crate to ethers crate
        let price = ethers::types::U256::from_str(&price.to_string())
            .map_err(|err| anyhow!("could not convert from web3::U256 to ethers::U256 {}", err))?;

        self.api
            .anvil_set_min_gas_price(price)
            .await
            .map_err(|err| anyhow!("could not set min gas price {:?}", err))
    }
}
