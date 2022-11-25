use anyhow::Error;
use bitski::Bitski;
use bitski_chain_models::networks::Network;
use bitski_provider::provider::BitskiProvider;
use web3::Web3;

use std::sync::Arc;

pub trait BitskiLocalExt {
    /// Sets up Bitski provider to talk to a local node, e.g. Anvil.
    fn new_local_mode(rpc_override: Option<String>) -> Self;

    /// Get a provider that will have a `localhost` node url, unless specifically overridden.
    fn get_local_provider<N>(&self, network: N) -> Result<BitskiProvider, Error>
    where
        N: TryInto<Network> + ToString;

    /// Get a Web3 provider that is connected to a local node.
    fn get_web3_test_mode<N>(&self, network: N) -> Result<Web3<BitskiProvider>, Error>
    where
        N: TryInto<Network> + ToString;
}

impl BitskiLocalExt for Bitski {
    fn new_local_mode(rpc_override: Option<String>) -> Self {
        let auth_token_provider = Arc::new(());
        Bitski {
            client_id: "TEST_CLIENT".to_string(),
            auth_token_provider,
            rpc_override,
        }
    }

    fn get_local_provider<N>(&self, network: N) -> Result<BitskiProvider, Error>
    where
        N: TryInto<Network> + ToString,
    {
        let network = new_local_network(network.to_string(), self.rpc_override.clone())?;

        let provider =
            BitskiProvider::new(&network, &self.client_id, self.auth_token_provider.clone());
        Ok(provider)
    }

    fn get_web3_test_mode<N>(&self, network: N) -> Result<Web3<BitskiProvider>, Error>
    where
        N: TryInto<Network> + ToString,
    {
        let provider = self.get_local_provider(network)?;
        Ok(Web3::new(provider))
    }
}

/// Create a new network for a local node. If the `rpc_url` is not None it will override the
/// default url for the node.
fn new_local_network(
    network_name: String,
    rpc_url: Option<String>,
) -> Result<Network, anyhow::Error> {
    let node = match network_name.as_str() {
        "anvil" => {
            let rpc_url = match rpc_url {
                Some(url) => url,
                None => "http://localhost:8545".to_string(),
            };
            Some(Network {
                rpc_url,
                chain_id: 31337,
            })
        }
        // TODO ganache and hardhat
        _ => None,
    };
    node.ok_or_else(|| anyhow::anyhow!("local network not configured"))
}
