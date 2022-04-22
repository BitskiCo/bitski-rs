use bitski_chain_models::networks::Network;
use jsonrpc_core::Call;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use web3::error::TransportError;
use web3::futures::future::BoxFuture;
use web3::{helpers, Error, RequestId, Transport};

#[derive(Debug, Clone)]
pub struct RestProvider {
    client: reqwest::Client,
    network: Network,
    client_id: String,
    id: Arc<AtomicUsize>,
}

impl RestProvider {
    pub fn new(network: Network, client_id: &dyn ToString) -> Self {
        RestProvider {
            client: reqwest::Client::new(),
            network,
            client_id: client_id.to_string(),
            id: Arc::new(AtomicUsize::new(0)),
        }
    }

    async fn send<T: DeserializeOwned>(
        client: &reqwest::Client,
        request: Call,
        network: Network,
        client_id: String,
    ) -> web3::error::Result<T> {
        let method_call = match request {
            Call::MethodCall(ref method_call) => method_call,
            _ => return Err(Error::Internal),
        };

        let url = format!(
            "{}/{}?params={}",
            network.rpc_url,
            method_call.method,
            serde_json::to_string(&method_call.params)?,
        );

        #[cfg(feature = "tracing")]
        tracing::debug!(
            "[id:{}] sending request: {:?} to {}",
            id,
            serde_json::to_string(&request)?,
            url
        );
        let response = client
            .get(url)
            .header("X-API-Key", client_id)
            .send()
            .await
            .map_err(|err| {
                Error::Transport(TransportError::Message(format!(
                    "failed to send request: {}",
                    err
                )))
            })?;
        let status = response.status();
        let response = response.bytes().await.map_err(|err| {
            Error::Transport(TransportError::Message(format!(
                "failed to read response bytes: {}",
                err
            )))
        })?;
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "[id:{}] received response: {:?}",
            id,
            String::from_utf8_lossy(&response)
        );
        if !status.is_success() {
            return Err(Error::Transport(TransportError::Code(status.as_u16())));
        }
        helpers::arbitrary_precision_deserialize_workaround(&response).map_err(|err| {
            Error::Transport(TransportError::Message(format!(
                "failed to deserialize response: {}",
                err
            )))
        })
    }
}

impl Transport for RestProvider {
    type Out = BoxFuture<'static, web3::error::Result<jsonrpc_core::Value>>;

    fn prepare(&self, method: &str, params: Vec<Value>) -> (RequestId, Call) {
        let id = self.id.fetch_add(1, Ordering::AcqRel);
        let request = helpers::build_request(id, method, params);
        (id, request)
    }

    fn send(&self, _id: RequestId, call: Call) -> Self::Out {
        let client = self.client.clone();
        let network = self.network.clone();
        let client_id = self.client_id.clone();
        Box::pin(async move {
            let output: jsonrpc_core::Value = Self::send(&client, call, network, client_id).await?;
            Ok(output)
        })
    }
}
