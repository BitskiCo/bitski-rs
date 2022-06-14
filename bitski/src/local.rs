use crate::Bitski;

use anyhow::Error;
use bitski_chain_models::networks::new_local_network;
use bitski_chain_models::networks::Network;
use bitski_provider::provider::BitskiProvider;
use web3::Web3;

use std::sync::Arc;

impl Bitski {
    /// Sets up Bitski provider to talk to a local node, e.g. Anvil.
    pub fn new_local_mode(rpc_override: Option<String>) -> Self {
        let auth_token_provider = Arc::new(());
        Bitski {
            client_id: "TEST_CLIENT".to_string(),
            auth_token_provider,
            rpc_override,
        }
    }

    /// Get a provider that will have a `localhost` node url, unless specifically overridden.
    pub fn get_local_provider<N>(&self, network: N) -> Result<BitskiProvider, Error>
    where
        N: TryInto<Network> + ToString,
    {
        let network = new_local_network(network.to_string(), self.rpc_override.clone())?;

        let provider =
            BitskiProvider::new(&network, &self.client_id, self.auth_token_provider.clone());
        Ok(provider)
    }

    /// Get a Web3 provider that is connected to a local node.
    pub fn get_web3_test_mode<N>(&self, network: N) -> Result<Web3<BitskiProvider>, Error>
    where
        N: TryInto<Network> + ToString,
    {
        let provider = self.get_local_provider(network)?;
        Ok(Web3::new(provider))
    }
}
