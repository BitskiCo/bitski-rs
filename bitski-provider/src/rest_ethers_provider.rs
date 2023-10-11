use crate::USER_AGENT;
use bitski_chain_models::networks::Network;
use ethers::prelude::{HttpClientError, JsonRpcClient};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct RestEthersProvider {
    client: reqwest::Client,
    pub network: Network,
    client_id: String,
}

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT.clone())
        .build()
        .expect("could not build REST client")
});

impl RestEthersProvider {
    pub fn new(network: Network, client_id: &dyn ToString) -> Self {
        RestEthersProvider {
            client: CLIENT.clone(),
            network,
            client_id: client_id.to_string(),
        }
    }

    async fn send<I: Debug + Serialize, T: DeserializeOwned>(
        &self,
        method: &str,
        params: I,
    ) -> Result<T, HttpClientError> {
        let url = format!(
            "{}/{}?params={}",
            self.network.rpc_url,
            method,
            serde_json::to_string(&params).map_err(|error| HttpClientError::SerdeJson {
                err: error,
                text: format!("{params:?}")
            })?,
        );

        let response = self
            .client
            .get(url)
            .header("X-API-Key", &self.client_id)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[async_trait::async_trait]
impl JsonRpcClient for RestEthersProvider {
    type Error = HttpClientError;

    async fn request<T: Debug + Serialize + Send + Sync, R: DeserializeOwned + Send>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, HttpClientError> {
        let output = self.send(method, params).await?;
        Ok(output)
    }
}
