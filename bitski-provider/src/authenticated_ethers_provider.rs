use crate::access_token_providers::AccessTokenProvider;
use crate::USER_AGENT;
use bitski_chain_models::networks::Network;
use ethers::prelude::*;
use reqwest::header::HeaderValue;
use reqwest::Client;
use reqwest::{header, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AuthenticatedEthersProvider {
    pub network: Network,
    pub client_id: String,
    pub auth_token_provider: Arc<dyn AccessTokenProvider + Sync + Send>,
}

impl AuthenticatedEthersProvider {
    pub fn new(
        network: Network,
        client_id: &dyn ToString,
        auth_token_provider: Arc<dyn AccessTokenProvider + Sync + Send>,
    ) -> Self {
        AuthenticatedEthersProvider {
            network,
            client_id: client_id.to_string(),
            auth_token_provider,
        }
    }

    async fn send<I: Debug + Serialize + Send + Sync, T: DeserializeOwned + Send>(
        &self,
        method: &str,
        params: I,
    ) -> Result<T, HttpClientError> {
        let access_token = self
            .auth_token_provider
            .get_access_token()
            .await
            .map_err(|error| {
                HttpClientError::JsonRpcError(JsonRpcError {
                    code: 403,
                    message: format!("{error}"),
                    data: None,
                })
            })?;
        self.send_with_auth(method, params, access_token).await
    }

    async fn send_with_auth<I: Debug + Serialize + Send + Sync, T: DeserializeOwned + Send>(
        &self,
        method: &str,
        params: I,
        token: String,
    ) -> Result<T, HttpClientError> {
        let auth_header_value = format!("Bearer {}", token).parse().map_err(|error| {
            HttpClientError::JsonRpcError(JsonRpcError {
                code: 403,
                message: format!("{error}"),
                data: None,
            })
        })?;

        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header_value);
        headers.insert("X-API-Key", HeaderValue::from_str(&self.client_id).unwrap());
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str(&USER_AGENT).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|error| {
                HttpClientError::JsonRpcError(JsonRpcError {
                    code: 500,
                    message: format!("{error}"),
                    data: None,
                })
            })?;

        let url: Url = self
            .network
            .rpc_url
            .parse()
            .expect("Failed to parse RPC URL");

        let transport = Http::new_with_client(url, client);
        JsonRpcClient::request(&transport, method, params).await
    }
}

#[async_trait::async_trait]
impl JsonRpcClient for AuthenticatedEthersProvider {
    type Error = HttpClientError;

    async fn request<T: Debug + Serialize + Send + Sync, R: DeserializeOwned + Send>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, HttpClientError> {
        self.send(method, params).await
    }
}
