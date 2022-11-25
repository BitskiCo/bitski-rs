use anvil::eth::EthApi;
use anvil::NodeConfig;
use anyhow::anyhow;

static DEFAULT_PORT: u16 = 8545;

pub mod anvil_config {
    pub use anvil::{Hardfork, NodeConfig};
    /// In the `web3` crate the `primitives_type` crate is a different version than in `ethers`.
    /// Hopefully they align at some point and we can remove this.
    pub use ethers::types::U256;
}

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
            ..Default::default()
        };

        let (api, handle) = anvil::spawn(config).await;
        Anvil {
            api,
            rpc_url: handle.http_endpoint(),
        }
    }

    /// Create a new Anvil node from a full config object.
    pub async fn new_from_config(config: NodeConfig) -> Self {
        let (api, handle) = anvil::spawn(config).await;
        Anvil {
            api,
            rpc_url: handle.http_endpoint(),
        }
    }

    /// Sets the minimum gas price for the node. It only works if the hardfork setting of the node
    /// is before London, which is when EIP-1559 transactions were introduced.
    pub async fn set_min_gas_price(&self, price: ethers::types::U256) -> Result<(), anyhow::Error> {
        self.api
            .anvil_set_min_gas_price(price)
            .await
            .map_err(|err| anyhow!("could not set min gas price {:?}", err))
    }
}
