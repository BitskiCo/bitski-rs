pub mod access_token_providers;
pub mod authenticated_provider;
pub mod provider;
pub mod rest_provider;

use once_cell::sync::Lazy;

pub(crate) static USER_AGENT: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/{}",
        option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")),
        env!("CARGO_PKG_VERSION")
    )
});
