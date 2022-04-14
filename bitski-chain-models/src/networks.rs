use crate::chains;

#[derive(Clone, Debug)]
pub struct Network {
    pub rpc_url: String,
    pub chain_id: u64,
}

impl TryFrom<&str> for Network {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "mainnet" {
            return Ok(Network {
                rpc_url: "https://api.bitski.com/v1/web3/mainnet".to_owned(),
                chain_id: 1,
            });
        }

        let chain = chains::chain_from_str(&value)?;

        Ok(Network {
            rpc_url: format!("https://api.bitski.com/v1/web3/{}", chain.chain_id),
            chain_id: chain.chain_id,
        })
    }
}
