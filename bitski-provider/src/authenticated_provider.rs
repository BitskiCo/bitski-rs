use crate::access_token_providers::AccessTokenProvider;
use bitski_chain_models::networks::Network;
use jsonrpc_core::futures::future::LocalBoxFuture;
use jsonrpc_core::Call;
use reqwest::header::HeaderValue;
use reqwest::{header, Client, Url};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use web3::futures::{FutureExt, TryFutureExt};
use web3::transports::Http;
use web3::{helpers, RequestId, Transport};

#[derive(Clone, Debug)]
pub struct AuthenticatedProvider {
    pub network: Network,
    pub client_id: String,
    pub auth_token_provider: Arc<dyn AccessTokenProvider>,
    id: Arc<AtomicUsize>,
}

impl AuthenticatedProvider {
    pub fn new(
        network: Network,
        client_id: &dyn ToString,
        auth_token_provider: Arc<dyn AccessTokenProvider>,
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
        headers.insert(header::AUTHORIZATION, auth_header_value);
        headers.insert("X-API-Key", HeaderValue::from_str(&client_id).unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_error| web3::error::Error::Internal)?;

        let transport = Http::with_client(client, url);
        transport.send(id, request).await
    }
}

impl Transport for AuthenticatedProvider {
    type Out = LocalBoxFuture<'static, web3::error::Result<jsonrpc_core::Value>>;

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
            .boxed_local()
    }
}
