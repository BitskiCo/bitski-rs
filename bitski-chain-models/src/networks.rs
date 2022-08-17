use crate::chains;

#[derive(Clone, Debug)]
pub struct Network {
    pub rpc_url: String,
    pub chain_id: u64,
}

impl TryFrom<&str> for Network {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chain = match value {
            "mainnet" => {
                return Ok(Network {
                    rpc_url: "https://api.bitski.com/v1/web3/mainnet".to_owned(),
                    chain_id: 1,
                })
            }
            // remap to the short name since 'goerli' isn't in the list
            "goerli" => chains::chain_from_str("gor"),
            _ => chains::chain_from_str(value),
        }?;

        Ok(Network {
            rpc_url: format!("https://api.bitski.com/v1/web3/{}", chain.chain_id),
            chain_id: chain.chain_id,
        })
    }
}

/// Create a new network for a local node. If the `rpc_url` is not None it will override the
/// default url for the node.
#[cfg(feature = "local")]
pub fn new_local_network(
    network_name: String,
    rpc_url: Option<String>,
) -> Result<Network, anyhow::Error> {
    let node = match network_name.as_str() {
        "anvil" => {
            let rpc_url = match rpc_url {
                Some(url) => url,
                None => "http://localhost:8545".to_string(),
            };
            Some(Network {
                rpc_url,
                chain_id: 31337,
            })
        }
        // TODO ganache and hardhat
        _ => None,
    };
    node.ok_or_else(|| anyhow::anyhow!("local network not configured"))
}

#[test]
fn test_chain_name_try_from() {
    let n = Network::try_from("goerli").expect("could not get goerli chain");
    assert_eq!(n.chain_id, 5);

    let n = Network::try_from("mainnet").expect("could not get mainnet chain");
    assert_eq!(n.chain_id, 1);

    let n = Network::try_from("polygon").expect("could not get polygon chain");
    assert_eq!(n.chain_id, 137);
}
