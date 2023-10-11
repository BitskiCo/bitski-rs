pub mod access_token_providers;
#[cfg(feature = "ethers")]
pub mod authenticated_ethers_provider;
pub mod authenticated_web3_provider;

#[cfg(feature = "ethers")]
pub mod ethers_provider;

#[cfg(feature = "ethers")]
pub mod rest_ethers_provider;
pub mod rest_web3_provider;
pub mod web3_provider;

use once_cell::sync::Lazy;

pub(crate) static USER_AGENT: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/{}",
        option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
        env!("CARGO_PKG_VERSION")
    )
});
