use anyhow::Error;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenResponse, TokenUrl};
use std::fmt::Debug;
use web3::futures::future::LocalBoxFuture;

pub trait AccessTokenProvider: Debug {
    fn get_access_token(&self) -> LocalBoxFuture<'static, Result<String, Error>>;
}

#[derive(Clone, Debug)]
pub struct ClientCredentialsAccessTokenProvider {
    client: BasicClient,
}

impl ClientCredentialsAccessTokenProvider {
    pub fn new(client_id: String, client_secret: String) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://account.bitski.com/oauth2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://account.bitski.com/oauth2/token".to_string()).unwrap()),
        );

        Self { client }
    }

    async fn get_access_token(client: BasicClient) -> Result<String, Error> {
        let response = client
            .exchange_client_credentials()
            .request_async(oauth2::reqwest::async_http_client)
            .await;
        match response {
            Ok(response) => Ok(response.access_token().secret().clone()),
            Err(error) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("Got an error exchanging client credentials: {:?}", error);
                Err(error.into())
            }
        }
    }
}

impl AccessTokenProvider for ClientCredentialsAccessTokenProvider {
    fn get_access_token(&self) -> LocalBoxFuture<'static, Result<String, Error>> {
        let client = self.client.clone();
        Box::pin(async move { Self::get_access_token(client).await })
    }
}

impl AccessTokenProvider for String {
    fn get_access_token(&self) -> LocalBoxFuture<'static, Result<String, Error>> {
        Box::pin(std::future::ready(Ok(self.clone())))
    }
}

impl AccessTokenProvider for () {
    fn get_access_token(&self) -> LocalBoxFuture<'static, Result<String, Error>> {
        Box::pin(std::future::ready(Err(Error::msg("Not signed in"))))
    }
}
