use crate::access_token_providers::AccessTokenProvider;
use bitski_chain_models::networks::Network;
use jsonrpc_core::futures::future::BoxFuture;
use jsonrpc_core::Call;
use reqwest::header::HeaderValue;
use reqwest::{header, Client, Url};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use web3::futures::{FutureExt, TryFutureExt};
use web3::transports::Http;
use web3::{helpers, BatchTransport, RequestId, Transport};

#[derive(Clone, Debug)]
pub struct AuthenticatedProvider {
    pub network: Network,
    pub client_id: String,
    pub auth_token_provider: Arc<dyn AccessTokenProvider + Sync + Send>,
    id: Arc<AtomicUsize>,
}

impl AuthenticatedProvider {
    pub fn new(
        network: Network,
        client_id: &dyn ToString,
        auth_token_provider: Arc<dyn AccessTokenProvider + Sync + Send>,
    ) -> Self {
        AuthenticatedProvider {
            network,
            client_id: client_id.to_string(),
            auth_token_provider,
            id: Arc::new(AtomicUsize::new(0)),
        }
    }

    async fn send_with_auth(
        url: Url,
        client_id: String,
        token: String,
        id: RequestId,
        request: Call,
    ) -> Result<jsonrpc_core::Value, web3::error::Error> {
        let auth_header_value = format!("Bearer {}", token)
            .parse()
            .map_err(|_error| web3::error::Error::Internal)?;

        let mut headers = header::HeaderMap::new();
        let user_agent = format!(
            "{}/{}",
            option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
            env!("CARGO_PKG_VERSION")
        );
        headers.insert(header::AUTHORIZATION, auth_header_value);
        headers.insert("X-API-Key", HeaderValue::from_str(&client_id).unwrap());

        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .map_err(|_error| web3::error::Error::Internal)?;

        let transport = Http::with_client(client, url);
        transport.send(id, request).await
    }

    async fn convert_to_batch(
        url: Url,
        client_id: String,
        requests: Vec<(RequestId, Call)>,
        token: String,
    ) -> web3::error::Result<Vec<web3::error::Result<jsonrpc_core::Value>>> {
        let mut results = Vec::new();
        for (id, call) in requests {
            let result =
                Self::send_with_auth(url.clone(), client_id.clone(), token.clone(), id, call).await;
            results.push(result);
        }
        Ok(results)
    }
}

impl Transport for AuthenticatedProvider {
    type Out = BoxFuture<'static, web3::error::Result<jsonrpc_core::Value>>;

    fn prepare(&self, method: &str, params: Vec<serde_json::value::Value>) -> (RequestId, Call) {
        let id = self.id.fetch_add(1, Ordering::AcqRel);
        let request = helpers::build_request(id, method, params);
        (id, request)
    }

    fn send(&self, id: RequestId, request: Call) -> Self::Out {
        let url = self
            .network
            .rpc_url
            .parse()
            .map_err(|_error| web3::error::Error::Internal)
            .unwrap();
        let client_id = self.client_id.clone();

        self.auth_token_provider
            .get_access_token()
            .map_err(|_error| web3::error::Error::Internal)
            .and_then(move |token| Self::send_with_auth(url, client_id, token, id, request))
            .boxed()
    }
}

impl BatchTransport for AuthenticatedProvider {
    type Batch =
        BoxFuture<'static, web3::error::Result<Vec<web3::error::Result<jsonrpc_core::Value>>>>;

    fn send_batch<T>(&self, requests: T) -> Self::Batch
    where
        T: IntoIterator<Item = (RequestId, Call)>,
    {
        let url = self
            .network
            .rpc_url
            .parse()
            .map_err(|_error| web3::error::Error::Internal)
            .unwrap();
        let client_id = self.client_id.clone();

        let requests_vec = requests.into_iter().collect();

        self.auth_token_provider
            .get_access_token()
            .map_err(|_error| web3::error::Error::Internal)
            .and_then(move |token: String| {
                Self::convert_to_batch(url, client_id, requests_vec, token).boxed()
            })
            .boxed()
    }
}
