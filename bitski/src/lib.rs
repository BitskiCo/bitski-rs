use anyhow::Error;
use bitski_chain_models::networks::Network;
use bitski_provider::access_token_providers::{
    AccessTokenProvider, ClientCredentialsAccessTokenProvider,
};
use bitski_provider::provider::BitskiProvider;
use std::sync::Arc;
use web3::Web3;

pub struct Bitski {
    client_id: String,
    auth_token_provider: Arc<dyn AccessTokenProvider>,
}

impl Bitski {
    /// Sets up Bitski to use client credentials for authentication.
    pub fn new(
        client_id: &dyn ToString,
        credential_id: &dyn ToString,
        client_secret: &dyn ToString,
        scopes: Option<Vec<String>>,
    ) -> Self {
        let auth_token_provider = Arc::new(ClientCredentialsAccessTokenProvider::new(
            credential_id.to_string(),
            client_secret.to_string(),
            scopes,
        ));
        Bitski {
            client_id: client_id.to_string(),
            auth_token_provider,
        }
    }

    /// Sets up Bitski with an existing access token
    pub fn new_with_access_token(client_id: &dyn ToString, access_token: &dyn ToString) -> Self {
        let auth_token_provider = Arc::new(access_token.to_string());
        Bitski {
            client_id: client_id.to_string(),
            auth_token_provider,
        }
    }

    /// Sets up Bitski without an access token provider
    pub fn new_unauthenticated(client_id: &dyn ToString) -> Self {
        let auth_token_provider = Arc::new(());
        Bitski {
            client_id: client_id.to_string(),
            auth_token_provider,
        }
    }

    pub fn from_env() -> Result<Self, Error> {
        let client_id = std::env::var("API_KEY").or_else(|_| std::env::var("CLIENT_ID"))?;
        let credential_id = std::env::var("CREDENTIAL_ID");
        let credential_secret = std::env::var("CREDENTIAL_SECRET");
        let scopes: Vec<String> = std::env::var("SCOPES")
            .unwrap_or_default()
            .split_terminator(',')
            .map(|s| s.to_string())
            .collect();
        let scopes = match scopes.is_empty() {
            true => None,
            false => Some(scopes),
        };

        match (credential_id, credential_secret) {
            (Ok(credential_id), Ok(credential_secret)) => Ok(Bitski::new(
                &client_id,
                &credential_id,
                &credential_secret,
                scopes,
            )),
            _ => Ok(Bitski::new_unauthenticated(&client_id)),
        }
    }

    pub fn get_provider<N: TryInto<Network>>(&self, network: N) -> Result<BitskiProvider, Error> {
        let network: Network = network
            .try_into()
            .map_err(|_error| Error::msg("Invalid network"))?;
        let provider =
            BitskiProvider::new(&network, &self.client_id, self.auth_token_provider.clone());
        Ok(provider)
    }

    pub async fn get_access_token(&self) -> Result<String, Error> {
        self.auth_token_provider.get_access_token().await
    }

    pub fn get_web3<N: TryInto<Network>>(&self, network: N) -> Result<Web3<BitskiProvider>, Error> {
        let provider = self.get_provider(network)?;
        Ok(Web3::new(provider))
    }
}
