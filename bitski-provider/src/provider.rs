use crate::access_token_providers::AccessTokenProvider;
use crate::authenticated_provider::AuthenticatedProvider;
use crate::rest_provider::RestProvider;
use crate::USER_AGENT;
use bitski_chain_models::networks::Network;
use jsonrpc_core::futures::future::BoxFuture;
use jsonrpc_core::Call;
use reqwest::header::HeaderValue;
use reqwest::{header, Client, Url};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use web3::futures::FutureExt;
use web3::transports::Http;
use web3::{helpers, BatchTransport, RequestId, Transport};

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
pub struct BitskiProvider {
    pub client_id: String,
    pub authenticated_provider: Arc<AuthenticatedProvider>,
    pub rest_provider: Arc<RestProvider>,
    pub http_provider: Arc<Http>,
    id: Arc<AtomicUsize>,
}

impl BitskiProvider {
    pub fn new(
        network: &Network,
        client_id: &dyn ToString,
        auth_token_provider: Arc<dyn AccessTokenProvider + Sync + Send>,
    ) -> Self {
        BitskiProvider {
            client_id: client_id.to_string(),
            authenticated_provider: Arc::new(AuthenticatedProvider::new(
                network.clone(),
                client_id,
                auth_token_provider,
            )),
            rest_provider: Arc::new(RestProvider::new(network.clone(), client_id)),
            http_provider: Self::http_provider(network, client_id),
            id: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn http_provider(network: &Network, client_id: &dyn ToString) -> Arc<Http> {
        let url: Url = network.rpc_url.parse().expect("Failed to parse RPC URL");

        let client_id = client_id.to_string();
        let mut headers = header::HeaderMap::new();

        if url.as_str().contains("api.bitski.com") {
            headers.insert("X-API-Key", HeaderValue::from_str(&client_id).unwrap());
        }

        let client = Client::builder()
            .user_agent(USER_AGENT.clone())
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Arc::new(Http::with_client(client, url))
    }
}

impl Transport for BitskiProvider {
    type Out = BoxFuture<'static, web3::error::Result<jsonrpc_core::Value>>;

    fn prepare(&self, method: &str, params: Vec<serde_json::value::Value>) -> (RequestId, Call) {
        let id = self.id.fetch_add(1, Ordering::AcqRel);
        let request = helpers::build_request(id, method, params);
        (id, request)
    }

    fn send(&self, id: RequestId, request: Call) -> Self::Out {
        match &request {
            Call::MethodCall(method_call)
                if AUTH_METHODS.contains(&method_call.method.as_str()) =>
            {
                self.authenticated_provider.send(id, request).boxed()
            }
            Call::MethodCall(method_call)
                if REST_METHODS.contains(&method_call.method.as_str()) =>
            {
                self.rest_provider.send(id, request).boxed()
            }
            _ => self.http_provider.send(id, request).boxed(),
        }
    }
}

impl BatchTransport for BitskiProvider {
    type Batch =
        BoxFuture<'static, web3::error::Result<Vec<web3::error::Result<jsonrpc_core::Value>>>>;

    fn send_batch<T>(&self, requests: T) -> Self::Batch
    where
        T: IntoIterator<Item = (RequestId, Call)>,
    {
        self.authenticated_provider.send_batch(requests)
    }
}
