use crate::access_token_providers::AccessTokenProvider;
use crate::authenticated_ethers_provider::AuthenticatedEthersProvider;
use crate::rest_ethers_provider::RestEthersProvider;
use crate::USER_AGENT;
use bitski_chain_models::networks::Network;
use cached::proc_macro::cached;
use ethers::prelude::{Http, HttpClientError, JsonRpcClient};
use reqwest::header::HeaderValue;
use reqwest::{header, Client, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

static AUTH_METHODS: &[&str] = &[
    "eth_sendTransaction",
    "eth_accounts",
    "eth_sign",
    "personal_sign",
    "eth_signTypedData",
    "eth_signTypedData_v3",
    "eth_signTypedData_v4",
];

static REST_METHODS: &[&str] = &[
    "eth_blockNumber",
    "eth_getBlockByNumber",
    "net_version",
    "eth_getLogs",
];

#[derive(Clone, Debug)]
pub struct BitskiEthersProvider {
    pub client_id: String,
    pub authenticated_provider: Arc<AuthenticatedEthersProvider>,
    pub rest_provider: Arc<RestEthersProvider>,
    pub http_provider: Arc<Http>,
}

impl BitskiEthersProvider {
    pub fn new<S: ToString>(
        network: &Network,
        client_id: &S,
        auth_token_provider: Arc<dyn AccessTokenProvider + Sync + Send>,
    ) -> Self {
        BitskiEthersProvider {
            client_id: client_id.to_string(),
            authenticated_provider: Arc::new(AuthenticatedEthersProvider::new(
                network.clone(),
                client_id,
                auth_token_provider,
            )),
            rest_provider: Arc::new(RestEthersProvider::new(network.clone(), client_id)),
            http_provider: http_provider(network.clone(), client_id.to_string()),
        }
    }
}

#[cached]
fn http_provider(network: Network, client_id: String) -> Arc<Http> {
    let url: Url = network.rpc_url.parse().expect("Failed to parse RPC URL");

    let mut headers = header::HeaderMap::new();

    if url.as_str().contains("api.bitski.com") {
        headers.insert("X-API-Key", HeaderValue::from_str(&client_id).unwrap());
    }

    let client = Client::builder()
        .user_agent(USER_AGENT.clone())
        .default_headers(headers)
        .build()
        .expect("Failed to build HTTP client");

    Arc::new(Http::new_with_client(url, client))
}

#[async_trait::async_trait]
impl JsonRpcClient for BitskiEthersProvider {
    type Error = HttpClientError;

    async fn request<T: Debug + Serialize + Send + Sync, R: DeserializeOwned + Send>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, HttpClientError> {
        let result = if AUTH_METHODS.contains(&method) {
            self.authenticated_provider.request(method, params).await?
        } else if REST_METHODS.contains(&method) {
            self.rest_provider.request(method, params).await?
        } else {
            self.http_provider.request(method, params).await?
        };

        Ok(result)
    }
}
